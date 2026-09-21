const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs/promises");
const { existsSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { join } = require("node:path");
const http = require("node:http");
const { createHash } = require("node:crypto");
const { target, checksum, download, install } = require("../scripts/install");

const asset = "yoo-linux-x86_64";
const binary = Buffer.from("native binary fixture\n");
const hash = createHash("sha256").update(binary).digest("hex");
const metadata = `${hash}  ${asset}\n`;

async function fixture(t, handler) {
  const root = await fs.mkdtemp(join(tmpdir(), "yoo-npm-test-"));
  const directory = join(root, "vendor");
  const server = http.createServer(handler);
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  t.after(async () => {
    server.closeAllConnections();
    await new Promise((resolve) => server.close(resolve));
    await fs.rm(root, { recursive: true, force: true });
  });
  // Only the injected transport uses HTTP. Production downloads still enforce
  // HTTPS, including redirects. No live release or external network is needed.
  const get = (url, options, callback) => http.get({
    hostname: "127.0.0.1", port: server.address().port, path: url.pathname,
    signal: options.signal
  }, callback);
  return { root, directory, options: { directory, platform: "linux", arch: "x64", localBinary: null,
    downloadOptions: { get, timeoutMs: 500, maxRedirects: 2 } } };
}

test("retains supported platforms and diagnoses unsupported platforms", () => {
  assert.equal(target("win32", "x64").exe, "yoo.exe");
  assert.equal(target("linux", "x64").asset, asset);
  assert.equal(target("darwin", "arm64").asset, "yoo-macos-aarch64");
  assert.throws(() => target("linux", "arm64"), /Unsupported platform.*linux-arm64/);
});

test("parses published checksum format and rejects ambiguous or malformed metadata", () => {
  assert.equal(checksum(metadata, asset), hash);
  assert.equal(checksum(`${hash.toUpperCase()} *${asset}\r\n`, asset), hash);
  for (const text of ["", "garbage", `${hash}  other`, metadata + metadata, `xyz  ${asset}`]) {
    assert.throws(() => checksum(text, asset), /metadata/);
  }
});

test("verifies release checksum, follows relative redirects, atomically replaces binary", async (t) => {
  let directory;
  const fixtureData = await fixture(t, (req, res) => {
    if (req.url.endsWith("SHA256SUMS")) return res.end(metadata);
    if (req.url !== "/binary") { res.writeHead(302, { Location: "/binary" }); return res.end(); }
    assert.equal(existsSync(join(directory, "yoo")), true);
    res.end(binary);
  });
  directory = fixtureData.directory;
  await fs.mkdir(directory);
  await fs.writeFile(join(directory, "yoo"), "previous executable");
  await install(fixtureData.options);
  assert.deepEqual(await fs.readFile(join(directory, "yoo")), binary);
  assert.deepEqual(await fs.readdir(directory), ["yoo"]);
  if (process.platform !== "win32") assert.equal((await fs.stat(join(directory, "yoo"))).mode & 0o777, 0o755);
});

const failures = [
  ["HTTP failure", (req, res) => { res.writeHead(404); res.end(); }, /HTTP 404/],
  ["redirect loop", (req, res) => { res.writeHead(302, { Location: req.url }); res.end(); }, /Too many redirects/],
  ["missing redirect location", (req, res) => { res.writeHead(302); res.end(); }, /missing Location/],
  ["insecure redirect", (req, res) => { res.writeHead(302, { Location: "http://example.invalid/" }); res.end(); }, /non-HTTPS/],
  ["request timeout", () => {}, /timeout/i],
  ["response timeout", (req, res) => { res.writeHead(200); res.write("partial"); }, /timeout/i],
  ["request error", (req) => req.socket.destroy(), /socket hang up|reset/i],
  ["response error", (req, res) => { res.writeHead(200, { "Content-Length": 999 }); res.write("partial"); setImmediate(() => res.destroy()); }, /aborted|premature|reset/i],
  ["malformed metadata", (req, res) => res.end("invalid checksum data"), /Malformed/],
  ["missing checksum", (req, res) => res.end(`${hash}  other\n`), /no checksum/],
  ["checksum mismatch", (req, res) => res.end(req.url.endsWith("SHA256SUMS") ? metadata : "corrupt"), /Checksum mismatch/],
  ["empty binary", (req, res) => res.end(req.url.endsWith("SHA256SUMS") ? metadata : ""), /empty/],
];
for (const [name, handler, expected] of failures) {
  test(`${name}: preserves existing executable and cleans staging`, async (t) => {
    const { directory, options } = await fixture(t, handler);
    await fs.mkdir(directory);
    await fs.writeFile(join(directory, "yoo"), "previous executable");
    await assert.rejects(install(options), expected);
    assert.equal(await fs.readFile(join(directory, "yoo"), "utf8"), "previous executable");
    assert.deepEqual(await fs.readdir(directory), ["yoo"]);
  });
}

test("failed first installation never publishes a partial executable", async (t) => {
  const { directory, options } = await fixture(t, (req, res) => {
    if (req.url.endsWith("SHA256SUMS")) return res.end(metadata);
    res.writeHead(200, { "Content-Length": 1000 }); res.write("partial");
    setImmediate(() => res.destroy());
  });
  await assert.rejects(install(options));
  assert.deepEqual(await fs.readdir(directory), []);
});

test("honors local binary override and cleans up filesystem failures", async (t) => {
  const { root, directory, options } = await fixture(t, () => assert.fail("local override must not download"));
  const localBinary = join(root, "local");
  await fs.writeFile(localBinary, binary);
  await install({ ...options, localBinary });
  assert.deepEqual(await fs.readFile(join(directory, "yoo")), binary);
  await assert.rejects(install({ ...options, localBinary: join(root, "missing") }), /ENOENT/);
  await fs.unlink(join(directory, "yoo"));
  await fs.mkdir(join(directory, "yoo"));
  await assert.rejects(install({ ...options, localBinary }), /EISDIR|EPERM|EACCES|EEXIST|ENOTEMPTY/);
  assert.deepEqual(await fs.readdir(directory), ["yoo"]);
});

test("download size limit and cancellation clean staging", async (t) => {
  const { directory, options } = await fixture(t, (req, res) => res.end(req.url.endsWith("SHA256SUMS") ? metadata : binary));
  await assert.rejects(install({ ...options, downloadOptions: { ...options.downloadOptions, maxBytes: 2 } }), /byte limit/);
  assert.deepEqual(await fs.readdir(directory), []);
  const controller = new AbortController();
  controller.abort(new Error("cancelled"));
  await assert.rejects(install({ ...options, signal: controller.signal }), /cancelled/);
});

test("standalone download rejects an insecure initial URL", async () => {
  await assert.rejects(download("http://example.invalid/file", "/unused"), /non-HTTPS/);
});

test("total deadline stops a continuously trickling response", async (t) => {
  const { directory, options } = await fixture(t, (req, res) => {
    res.writeHead(200);
    const interval = setInterval(() => res.write("x"), 10);
    res.on("close", () => clearInterval(interval));
  });
  const started = Date.now();
  await assert.rejects(install(options), /timeout/i);
  assert.ok(Date.now() - started < 3000);
  assert.deepEqual(await fs.readdir(directory), []);
});

test("cancellation during a response cleans open files before removing staging", async (t) => {
  const controller = new AbortController();
  const { directory, options } = await fixture(t, (req, res) => {
    if (req.url.endsWith("SHA256SUMS")) return res.end(metadata);
    res.write(binary.subarray(0, 2));
    setImmediate(() => controller.abort(new Error("cancelled during download")));
  });
  await assert.rejects(install({ ...options, signal: controller.signal }), /cancelled during download/);
  assert.deepEqual(await fs.readdir(directory), []);
});
