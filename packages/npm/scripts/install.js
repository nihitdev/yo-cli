const { createWriteStream } = require("node:fs");
const fs = require("node:fs/promises");
const https = require("node:https");
const { createHash } = require("node:crypto");
const { join } = require("node:path");
const { Transform } = require("node:stream");
const { pipeline } = require("node:stream/promises");
const { version } = require("../package.json");

const repo = "https://github.com/nihitdev/yo-cli";
const vendorDir = join(__dirname, "..", "vendor");
const targets = {
  "win32-x64": { asset: "yoo-windows-x86_64.exe", exe: "yoo.exe" },
  "linux-x64": { asset: "yoo-linux-x86_64", exe: "yoo" },
  "darwin-arm64": { asset: "yoo-macos-aarch64", exe: "yoo" }
};

function target(platform = process.platform, arch = process.arch) {
  const key = `${platform}-${arch}`;
  if (!Object.hasOwn(targets, key)) {
    throw new Error(`Unsupported platform for @nihit_dev/yoo: ${key}; supported: ${Object.keys(targets).join(", ")}. Use cargo install yoo.`);
  }
  return targets[key];
}

// One deadline covers DNS, TLS, redirects and the complete body, including a
// server that continuously sends data slowly enough to evade an idle timeout.
async function download(url, destination, {
  get = https.get, timeoutMs = 60_000, maxRedirects = 5,
  maxBytes = 128 * 1024 * 1024, signal
} = {}) {
  const controller = new AbortController();
  const abort = () => controller.abort(signal.reason);
  if (signal?.aborted) abort();
  else signal?.addEventListener("abort", abort, { once: true });
  const timer = setTimeout(() => controller.abort(new Error(`Download timeout after ${timeoutMs} ms`)), timeoutMs);
  let response;
  try {
    let current = new URL(url);
    for (let redirects = 0; ; redirects++) {
      if (current.protocol !== "https:") throw new Error(`Refusing non-HTTPS download: ${current.protocol}`);
      controller.signal.throwIfAborted();
      response = await new Promise((resolve, reject) => {
        const request = get(current, { signal: controller.signal }, resolve);
        request.on("error", reject);
        request.setTimeout(timeoutMs, () => request.destroy(new Error(`Request timeout after ${timeoutMs} ms`)));
      });
      if (![301, 302, 303, 307, 308].includes(response.statusCode)) break;
      const location = response.headers.location;
      response.destroy();
      if (redirects >= maxRedirects) throw new Error(`Too many redirects (limit ${maxRedirects})`);
      if (!location) throw new Error("Redirect response is missing Location");
      current = new URL(location, current);
    }
    if (response.statusCode !== 200) throw new Error(`HTTP ${response.statusCode} downloading ${current.pathname}`);
    let bytes = 0;
    const hash = createHash("sha256");
    const validate = new Transform({
      transform(chunk, encoding, callback) {
        bytes += chunk.length;
        if (bytes > maxBytes) return callback(new Error(`Download exceeds ${maxBytes} byte limit`));
        hash.update(chunk);
        callback(null, chunk);
      }
    });
    await pipeline(response, validate, createWriteStream(destination, { flags: "wx", mode: 0o600 }), { signal: controller.signal });
    if (bytes === 0) throw new Error("Downloaded file is empty");
    return hash.digest("hex");
  } catch (error) {
    const reason = controller.signal.aborted ? controller.signal.reason : error;
    throw new Error(`Could not download ${url}: ${reason.message || reason}`, { cause: error });
  } finally {
    clearTimeout(timer);
    signal?.removeEventListener("abort", abort);
    response?.destroy();
  }
}

function checksum(metadata, asset) {
  let expected;
  for (const line of metadata.split(/\r?\n/)) {
    if (!line.trim()) continue;
    const match = /^([a-fA-F0-9]{64}) [ *]([^\r\n]+)$/.exec(line);
    if (!match) throw new Error("Malformed SHA256SUMS metadata: expected sha256sum records");
    if (match[2] === asset) {
      if (expected) throw new Error(`Malformed SHA256SUMS metadata: duplicate entry for ${asset}`);
      expected = match[1].toLowerCase();
    }
  }
  if (!expected) throw new Error(`SHA256SUMS metadata has no checksum for ${asset}`);
  return expected;
}

async function install({
  directory = vendorDir, platform = process.platform, arch = process.arch,
  localBinary = process.env.YOO_BINARY_PATH, releaseVersion = version,
  downloadOptions = {}, signal
} = {}) {
  const selected = target(platform, arch);
  let staging;
  let failure;
  try {
    signal?.throwIfAborted();
    await fs.mkdir(directory, { recursive: true });
    // Same filesystem as the final executable, so rename is atomic. Never
    // truncate/remove an existing working installation before validation.
    staging = await fs.mkdtemp(join(directory, ".yoo-install-"));
    const binaryPath = join(staging, selected.exe);
    if (localBinary) {
      const stat = await fs.stat(localBinary);
      if (!stat.isFile() || stat.size === 0) throw new Error(`YOO_BINARY_PATH is not a nonempty regular file: ${localBinary}`);
      await fs.copyFile(localBinary, binaryPath);
    } else {
      const base = `${repo}/releases/download/v${releaseVersion}`;
      const metadataPath = join(staging, "SHA256SUMS");
      await download(`${base}/SHA256SUMS`, metadataPath, { ...downloadOptions, signal, maxBytes: 1024 * 1024 });
      const expected = checksum(await fs.readFile(metadataPath, "utf8"), selected.asset);
      const actual = await download(`${base}/${selected.asset}`, binaryPath, { ...downloadOptions, signal });
      if (actual !== expected) throw new Error(`Checksum mismatch for ${selected.asset}: expected ${expected}, received ${actual}`);
    }
    if (platform !== "win32") await fs.chmod(binaryPath, 0o755);
    signal?.throwIfAborted();
    await fs.rename(binaryPath, join(directory, selected.exe));
  } catch (error) {
    failure = new Error(`Could not install yoo into ${directory}: ${error.message}`, { cause: error });
  } finally {
    if (staging) {
      try { await fs.rm(staging, { recursive: true, force: true }); }
      catch (error) {
        failure = new Error(`${failure ? `${failure.message}; ` : ""}could not clean temporary directory ${staging}: ${error.message}`, { cause: error });
      }
    }
  }
  if (failure) throw failure;
}

if (require.main === module) {
  const controller = new AbortController();
  const interrupt = () => controller.abort(new Error("Installation interrupted (SIGINT)"));
  const terminate = () => controller.abort(new Error("Installation interrupted (SIGTERM)"));
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  install({ signal: controller.signal }).catch((error) => {
    console.error(error.message);
    process.exitCode = 1;
  }).finally(() => {
    process.removeListener("SIGINT", interrupt);
    process.removeListener("SIGTERM", terminate);
  });
}

module.exports = { target, download, checksum, install };
