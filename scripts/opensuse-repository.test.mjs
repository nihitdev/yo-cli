import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, delimiter } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
const fixtures = join(root, 'scripts/fixtures/apt-newline');
// Reuse public OpenPGP fixtures. RPM/build/signing commands are mocked;
// repository signature verification uses the real gpgv and no private keys.
for (const scenario of ['valid', 'tampered']) {
  test(`openSUSE verifies metadata using a seekable keyring: ${scenario}`, t => {
    const dir = mkdtempSync(join(tmpdir(), 'yoo-opensuse-'));
    t.after(() => rmSync(dir, { recursive: true, force: true }));
    const bin = join(dir, 'bin');
    const output = join(dir, 'repo');
    mkdirSync(bin);
    mkdirSync(output);
    writeFileSync(join(output, 'sentinel'), 'previous repository');
    const rpm = join(dir, 'yoo-1.2.3-1.x86_64.rpm');
    writeFileSync(rpm, 'RPM fixture');
    const calls = join(dir, 'rpm-calls');
    const tool = `#!/usr/bin/env node
const fs = require('node:fs');
const path = require('node:path');
const args = process.argv.slice(2);
const tool = path.basename(process.argv[1]);
if (tool === 'rpm') {
  fs.appendFileSync(process.env.RPM_CALLS, JSON.stringify(args) + '\\n');
  if (args[0] === '-qp') process.stdout.write({ '%{NAME}': 'yoo', '%{ARCH}': 'x86_64', '%{VERSION}-%{RELEASE}': '1.2.3-1' }[args.at(-2)]);
} else if (tool === 'gpg') {
  if (args.includes('--show-keys')) console.log('fpr:::::::::' + 'A'.repeat(40) + ':');
  else if (args.includes('--export')) process.stdout.write(fs.readFileSync(path.join(process.env.FIXTURES, 'public-key.gpg')));
  else if (args.includes('--detach-sign')) fs.copyFileSync(path.join(process.env.FIXTURES, 'Release.gpg'), args[args.indexOf('--output') + 1]);
  else process.exit(1);
} else if (tool === 'createrepo_c') {
  const directory = path.join(args.at(-1), 'repodata');
  fs.mkdirSync(directory, { recursive: true });
  fs.writeFileSync(path.join(directory, 'repomd.xml'), process.env.SCENARIO === 'tampered' ? 'invalid metadata' : fs.readFileSync(path.join(process.env.FIXTURES, 'Release')));
} else process.exit(1);
`;
    for (const name of ['rpm', 'gpg', 'createrepo_c']) writeFileSync(join(bin, name), tool, { mode: 0o755 });
    const result = spawnSync('bash', ['scripts/opensuse-repository.sh', rpm, output], {
      cwd: root, encoding: 'utf8', env: {
        ...process.env, PATH: `${bin}${delimiter}${process.env.PATH}`, GNUPGHOME: dir,
        FIXTURES: fixtures, SCENARIO: scenario, RPM_CALLS: calls,
        RPM_SIGNING_KEY_ID: 'A'.repeat(40), RPM_SIGNING_PASSPHRASE: 'fixture',
      },
    });
    assert.equal(result.status === 0, scenario === 'valid', result.stderr);
    const rpmCalls = readFileSync(calls, 'utf8').trim().split('\n').map(JSON.parse);
    assert.ok(rpmCalls.findIndex(x => x[0] === '--import') < rpmCalls.findIndex(x => x.includes('--resign')));
    assert.ok(!existsSync(join(output, 'verification-keyring.gpg')));
    if (scenario === 'valid') {
      assert.match(result.stderr, /Good signature/);
      assert.deepEqual(readFileSync(join(output, 'repodata/repomd.xml')), readFileSync(join(fixtures, 'Release')));
    } else {
      assert.match(result.stderr, /BAD signature/);
      assert.ok(!existsSync(join(output, 'repodata')));
      assert.equal(readFileSync(join(output, 'sentinel'), 'utf8'), 'previous repository');
    }
  });
}
