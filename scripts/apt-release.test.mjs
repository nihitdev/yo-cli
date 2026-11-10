import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, copyFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, delimiter } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
const fixtures = join(root, 'scripts/fixtures/apt-newline');
function temporary(t) {
  const dir = mkdtempSync(join(tmpdir(), 'yoo-apt-release-'));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  return dir;
}
function verify(dir, ...args) {
  return spawnSync('gpgv', ['--keyring', join(fixtures, 'public-key.gpg'), ...args], {
    encoding: 'utf8', env: { ...process.env, GNUPGHOME: dir },
  });
}

test('real valid APT signatures reproduce the terminal-newline mismatch', t => {
  const dir = temporary(t);
  const extracted = join(dir, 'Release');
  const clear = verify(dir, '--output', extracted, join(fixtures, 'InRelease'));
  assert.equal(clear.status, 0, clear.stderr);
  const detached = verify(dir, join(fixtures, 'Release.gpg'), join(fixtures, 'Release'));
  assert.equal(detached.status, 0, detached.stderr);
  const release = readFileSync(join(fixtures, 'Release'));
  assert.equal(release.at(-1), 10);
  assert.deepEqual(readFileSync(extracted), release.subarray(0, -1));

  // Removing LF from an already signed Release cannot repair its signature.
  const altered = verify(dir, join(fixtures, 'Release.gpg'), extracted);
  assert.notEqual(altered.status, 0);
});

test('strict APT validation rejects separately valid signatures with different payload bytes', t => {
  const dir = temporary(t);
  const repo = join(dir, 'repo');
  mkdirSync(join(repo, 'dists/stable'), { recursive: true });
  copyFileSync(join(fixtures, 'public-key.gpg'), join(repo, 'yoo-archive-keyring.gpg'));
  for (const name of ['InRelease', 'Release', 'Release.gpg']) {
    copyFileSync(join(fixtures, name), join(repo, 'dists/stable', name));
  }
  const result = spawnSync('bash', ['scripts/apt-validate.sh', repo], {
    cwd: root, encoding: 'utf8', env: { ...process.env, GNUPGHOME: dir },
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stdout + result.stderr, /EOF|differ/);
  assert.match(result.stderr, /Good signature/);
});

// Exercise the full publisher without private keys or native Debian tools.
// Mock signing records its input; mock verification checks those exact bytes
// and models GnuPG's cleartext extraction, established by the real test above.
for (const [scenario, ending] of [['no-newline', ''], ['one-newline', '\n'], ['blank-lines', '\n\n'], ['different-payload', '\n'], ['bad-signature', '\n']]) {
  test(`APT publisher canonical Release bytes: ${scenario}`, t => {
    const dir = temporary(t);
    const bin = join(dir, 'bin');
    const out = join(dir, 'repo');
    mkdirSync(bin);
    mkdirSync(join(out, 'dists/stable'), { recursive: true });
    const sentinel = Buffer.from('previous signed artifact');
    for (const name of ['Release', 'Release.gpg', 'InRelease']) writeFileSync(join(out, 'dists/stable', name), sentinel);
    const deb = join(dir, 'yoo.deb');
    writeFileSync(deb, 'package fixture');
    const release = 'Origin: yoo\nSuite: stable\nComponents: main';
    const releaseFile = join(dir, 'Release.input');
    writeFileSync(releaseFile, release + ending);
    const calls = join(dir, 'signing-inputs.jsonl');
    const tool = `#!/usr/bin/env node
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const args = process.argv.slice(2);
const tool = path.basename(process.argv[1]);
const hash = data => crypto.createHash('sha256').update(data).digest('hex');
if (tool === 'dpkg-deb') {
  const value = { Package: 'yoo', Architecture: 'amd64', Version: '1.2.3-1' }[args.at(-1)];
  if (!value) process.exit(1);
  console.log(value);
} else if (tool === 'apt-ftparchive') {
  process.stdout.write(args[0] === 'packages' ? 'Package: yoo\\nVersion: 1.2.3-1\\n' : fs.readFileSync(process.env.RELEASE_INPUT));
} else if (tool === 'gpg') {
  if (args.includes('--export')) { process.stdout.write('public key fixture'); process.exit(0); }
  const input = fs.readFileSync(args.at(-1));
  const mode = args.includes('--clearsign') ? 'clear' : 'detached';
  fs.appendFileSync(process.env.SIGNING_INPUTS, JSON.stringify({ mode, input: input.toString() }) + '\\n');
  let content = input.toString();
  if (mode === 'clear') content = content.replace(/\\n$/, '');
  if (mode === 'clear' && process.env.SCENARIO === 'different-payload') content += '\\n';
  fs.writeFileSync(args[args.indexOf('--output') + 1], JSON.stringify({ content, hash: hash(Buffer.from(content)) }));
} else if (tool === 'gpgv') {
  const clear = args.includes('--output');
  const signature = JSON.parse(fs.readFileSync(clear ? args.at(-1) : args.at(-2)));
  const content = clear ? Buffer.from(signature.content) : fs.readFileSync(args.at(-1));
  if (hash(content) !== signature.hash || process.env.SCENARIO === 'bad-signature') process.exit(1);
  if (clear) fs.writeFileSync(args[args.indexOf('--output') + 1], content);
} else process.exit(1);
`;
    for (const name of ['dpkg-deb', 'apt-ftparchive', 'gpg', 'gpgv']) writeFileSync(join(bin, name), tool, { mode: 0o755 });
    const result = spawnSync('bash', ['scripts/apt-repository.sh', deb, out], {
      cwd: root, encoding: 'utf8', env: {
        ...process.env, PATH: `${bin}${delimiter}${process.env.PATH}`,
        APT_SIGNING_FINGERPRINT: 'A'.repeat(40), APT_EXPECTED_VERSION: '1.2.3',
        SIGNING_INPUTS: calls, RELEASE_INPUT: releaseFile, SCENARIO: scenario,
      },
    });
    const fails = ['different-payload', 'bad-signature'].includes(scenario);
    assert.equal(result.status === 0, !fails, result.stdout + result.stderr);
    const signed = readFileSync(calls, 'utf8').trim().split('\n').map(JSON.parse);
    assert.deepEqual(signed.map(x => x.mode), ['detached', 'clear']);
    assert.ok(signed.every(x => x.input === release), 'both signers receive the same canonical bytes');
    if (fails) {
      for (const name of ['Release', 'Release.gpg', 'InRelease']) assert.deepEqual(readFileSync(join(out, 'dists/stable', name)), sentinel);
    } else {
      const content = readFileSync(join(out, 'dists/stable/Release'), 'utf8');
      assert.equal(content, release);
      for (const name of ['Release.gpg', 'InRelease']) assert.equal(JSON.parse(readFileSync(join(out, 'dists/stable', name))).content, content);
    }
  });
}
