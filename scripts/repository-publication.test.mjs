import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync, copyFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, delimiter } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
function directory(t) {
  const dir = mkdtempSync(join(tmpdir(), 'yoo-publication-'));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  return dir;
}

// Mock Git completely: these tests never create commits or contact a remote.
for (const [scenario, status, pushes, aborts] of [
  ['success', 0, 1, 0], ['retry', 0, 2, 0], ['changed', 1, 0, 0],
  ['conflict', 1, 0, 1], ['rejected', 1, 5, 0],
]) {
  test(`repository publication: ${scenario}`, t => {
    const dir = directory(t);
    const bin = join(dir, 'bin');
    mkdirSync(bin);
    writeFileSync(join(bin, 'git'), `#!/usr/bin/env node
const fs = require('node:fs');
const args = process.argv.slice(2);
const calls = fs.existsSync(process.env.CALLS) ? JSON.parse(fs.readFileSync(process.env.CALLS)) : [];
calls.push(args);
fs.writeFileSync(process.env.CALLS, JSON.stringify(calls));
const scenario = process.env.SCENARIO;
if (args[0] === 'rev-parse') console.log('base');
if (args[0] === 'diff' && args.includes('FETCH_HEAD') && scenario === 'changed') process.exit(1);
if (args[0] === 'rebase' && args.includes('--onto') && scenario === 'conflict') process.exit(1);
if (args[0] === 'push' && (scenario === 'rejected' || (scenario === 'retry' && calls.filter(x => x[0] === 'push').length === 1))) process.exit(1);
`, { mode: 0o755 });
    const callsFile = join(dir, 'calls.json');
    const result = spawnSync('bash', ['scripts/push-repository-update.sh', 'site/public/apt'], {
      cwd: root, encoding: 'utf8',
      env: { ...process.env, PATH: `${bin}${delimiter}${process.env.PATH}`, CALLS: callsFile, SCENARIO: scenario, BRANCH: 'main' },
    });
    assert.equal(result.status, status, result.stderr);
    const calls = JSON.parse(readFileSync(callsFile));
    assert.equal(calls.filter(x => x[0] === 'push').length, pushes);
    assert.equal(calls.filter(x => x[0] === 'rebase' && x[1] === '--abort').length, aborts);
    assert.ok(!calls.flat().some(x => x.startsWith('--force')));
    assert.ok(calls.filter(x => x[0] === 'push').every(x => x[2] === 'HEAD:refs/heads/main'));
  });
}

for (const scenario of ['success', 'wrong-tag', 'missing-package', 'missing-binary']) {
  test(`Void build orchestration: ${scenario}`, t => {
    const dir = directory(t);
    const workspace = join(dir, 'workspace');
    const packages = join(dir, 'void-packages');
    for (const sub of ['target', 'packaging/void']) mkdirSync(join(workspace, sub), { recursive: true });
    mkdirSync(packages);
    writeFileSync(join(workspace, 'Cargo.toml'), '[package]\nversion = "1.2.3"\n');
    copyFileSync(join(root, 'packaging/void/template.in'), join(workspace, 'packaging/void/template.in'));
    const source = 'source archive fixture';
    writeFileSync(join(workspace, 'target/yoo-void-source.tar.gz'), source);
    // Stand in for the native compiler/package builder; archive validation is real.
    writeFileSync(join(packages, 'xbps-src'), `#!/usr/bin/env bash
set -eu
if [[ "$1" == binary-bootstrap ]]; then exit 0; fi
[[ "$*" == '-Q pkg yoo' ]]
[[ "$SCENARIO" != missing-package ]] || exit 0
mkdir -p hostdir/binpkgs fixture/usr/bin
if [[ "$SCENARIO" != missing-binary ]]; then printf binary > fixture/usr/bin/yoo; fi
tar -cf hostdir/binpkgs/yoo-1.2.3_1.x86_64.xbps -C fixture .
`, { mode: 0o755 });
    const result = spawnSync('bash', ['scripts/void-build.sh'], {
      cwd: root, encoding: 'utf8', env: { ...process.env, WORKSPACE: workspace, VOID_PACKAGES_DIR: packages, SCENARIO: scenario, RELEASE_TAG: scenario === 'wrong-tag' ? 'v9.9.9' : 'v1.2.3' },
    });
    assert.equal(result.status === 0, scenario === 'success', result.stderr);
    if (scenario === 'wrong-tag') {
      assert.ok(!existsSync(join(packages, 'srcpkgs/yoo/template')));
      return;
    }
    const template = readFileSync(join(packages, 'srcpkgs/yoo/template'), 'utf8');
    assert.ok(template.includes(`checksum=${createHash('sha256').update(source).digest('hex')}`));
    assert.ok(template.includes('version=1.2.3'));
    assert.equal(readFileSync(join(packages, 'hostdir/sources/yoo-1.2.3/v1.2.3.tar.gz'), 'utf8'), source);
    if (scenario === 'success') assert.ok(existsSync(join(workspace, 'dist-void/yoo-1.2.3_1.x86_64.xbps')));
  });
}
