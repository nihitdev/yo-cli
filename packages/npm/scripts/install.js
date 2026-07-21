const { createWriteStream, chmodSync, copyFileSync, existsSync, mkdirSync } = require("node:fs");
const { get } = require("node:https");
const { basename, join } = require("node:path");
const { version } = require("../package.json");

const repo = "https://github.com/nihitdev/yo-cli";
const vendorDir = join(__dirname, "..", "vendor");

const targets = {
  "win32-x64": { asset: "yoo-windows-x86_64.exe", exe: "yoo.exe" },
  "linux-x64": { asset: "yoo-linux-x86_64", exe: "yoo" },
  "darwin-arm64": { asset: "yoo-macos-aarch64", exe: "yoo" }
};

function target() {
  const key = `${process.platform}-${process.arch}`;
  const match = targets[key];

  if (!match) {
    throw new Error(`Unsupported platform for @nihitde_v/yoo: ${key}`);
  }

  return match;
}

function download(url, destination) {
  return new Promise((resolve, reject) => {
    get(url, (response) => {
      if ([301, 302, 303, 307, 308].includes(response.statusCode)) {
        response.resume();
        download(response.headers.location, destination).then(resolve, reject);
        return;
      }

      if (response.statusCode !== 200) {
        response.resume();
        reject(new Error(`Failed to download ${basename(destination)}: HTTP ${response.statusCode}`));
        return;
      }

      const file = createWriteStream(destination, { mode: 0o755 });
      response.pipe(file);
      file.on("finish", () => file.close(resolve));
      file.on("error", reject);
    }).on("error", reject);
  });
}

async function main() {
  mkdirSync(vendorDir, { recursive: true });

  const selected = target();
  const binaryPath = join(vendorDir, selected.exe);
  const localBinary = process.env.YOO_BINARY_PATH;

  if (localBinary) {
    if (!existsSync(localBinary)) {
      throw new Error(`YOO_BINARY_PATH does not exist: ${localBinary}`);
    }

    copyFileSync(localBinary, binaryPath);
  } else {
    const url = `${repo}/releases/download/v${version}/${selected.asset}`;
    await download(url, binaryPath);
  }

  if (process.platform !== "win32") {
    chmodSync(binaryPath, 0o755);
  }
}

main().catch((error) => {
  console.error(error.message);
  process.exit(1);
});
