#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const { existsSync } = require("node:fs");
const { join } = require("node:path");

const exe = process.platform === "win32" ? "yoo.exe" : "yoo";
const binary = join(__dirname, "..", "vendor", exe);

if (!existsSync(binary)) {
  console.error("yoo binary is missing. Try reinstalling @nihitdev/yoo.");
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 0);
