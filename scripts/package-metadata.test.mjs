import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { delimiter, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
const version = spawnSync('bash', ['scripts/package-version.sh'], {
  cwd: root, encoding: 'utf8',
});
assert.equal(version.status, 0, version.stderr);
const releaseVersion = version.stdout.trim();
const sha256 = content => createHash('sha256').update(content).digest('hex');

function fixture(t, includeExe = true) {
  const directory = mkdtempSync(join(tmpdir(), 'yoo-metadata-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const assets = join(directory, 'assets');
  const output = join(directory, 'output');
  const bin = join(directory, 'bin');
  mkdirSync(assets);
  mkdirSync(bin);
  // Only source-archive downloading is mocked; metadata generation and hashing
  // run unchanged. The test never contacts the network.
  writeFileSync(join(bin, 'curl'), `#!/usr/bin/env bash
set -eu
while [[ $# -gt 0 ]]; do
  if [[ "$1" == --output ]]; then
    printf 'source archive fixture' > "$2"
    exit 0
  fi
  shift
done
exit 1
`, { mode: 0o755 });
  for (const platform of ['linux-x86_64', 'macos-aarch64']) {
    writeFileSync(join(assets, `yoo-v${releaseVersion}-${platform}.tar.gz`), platform);
  }
  writeFileSync(join(assets, `yoo-v${releaseVersion}-windows-x86_64.zip`), 'zip fixture');
  if (includeExe) writeFileSync(join(assets, 'yoo-windows-x86_64.exe'), 'exe fixture');
  const result = spawnSync('bash', ['scripts/package-metadata.sh', assets, output], {
    cwd: root,
    encoding: 'utf8',
    env: { ...process.env, PATH: `${bin}${delimiter}${process.env.PATH}` },
  });
  return { result, output, directory };
}

test('Chocolatey verifies the raw executable while WinGet and Scoop verify the ZIP', t => {
  const { result, output } = fixture(t);
  assert.equal(result.status, 0, result.stderr);
  const installer = readFileSync(join(output, 'chocolatey/tools/chocolateyinstall.ps1'), 'utf8');
  assert.ok(installer.includes(`/v${releaseVersion}/yoo-windows-x86_64.exe'`));
  assert.ok(installer.includes(`-Checksum '${sha256('exe fixture')}'`));
  assert.ok(!installer.includes(sha256('zip fixture')));
  const winget = readFileSync(join(output, 'winget/nihitdev.yoo.installer.yaml'), 'utf8');
  assert.ok(winget.includes(`InstallerSha256: ${sha256('zip fixture')}`));
  const scoop = JSON.parse(readFileSync(join(output, 'scoop/yoo.json'), 'utf8'));
  assert.equal(scoop.hash, sha256('zip fixture'));
});

test('missing raw executable fails instead of emitting an unusable Chocolatey installer', t => {
  const { result } = fixture(t, false);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /yoo-windows-x86_64\.exe/);
});

test('the release bundle contains Chocolatey metadata and its installer', t => {
  const { result, output, directory } = fixture(t);
  assert.equal(result.status, 0, result.stderr);
  mkdirSync(join(directory, 'release'));
  mkdirSync(join(directory, 'target'));
  symlinkSync(output, join(directory, 'target/package-metadata'), 'dir');
  // Exercise the actual tar command used by the release workflow.
  const workflow = readFileSync(join(root, '.github/workflows/release.yml'), 'utf8');
  const command = workflow.split('\n').find(line => line.trim().startsWith('tar -czf release/yoo-packaging.tar.gz'));
  assert.ok(command, 'release workflow must create the packaging bundle');
  const bundle = spawnSync('bash', ['-c', command.trim()], { cwd: directory, encoding: 'utf8' });
  assert.equal(bundle.status, 0, bundle.stderr);
  const contents = spawnSync('tar', ['-tzf', join(directory, 'release/yoo-packaging.tar.gz')], { encoding: 'utf8' });
  assert.equal(contents.status, 0, contents.stderr);
  const files = contents.stdout.split('\n');
  assert.ok(files.includes('chocolatey/yoo.nuspec'));
  assert.ok(files.includes('chocolatey/tools/chocolateyinstall.ps1'));
});
