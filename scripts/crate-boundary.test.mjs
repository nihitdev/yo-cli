import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
test('crate include patterns exclude nested dependency documentation and build output', t => {
  const dir = mkdtempSync(join(tmpdir(), 'yoo-crate-boundary-'));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const manifest = readFileSync(join(root, 'Cargo.toml'), 'utf8');
  const include = manifest.match(/^include = \[[\s\S]*?^\]/m)?.[0];
  assert.ok(include);
  writeFileSync(join(dir, 'Cargo.toml'), `[package]\nname = "yoo-package-boundary-fixture"\nversion = "0.0.0"\nedition = "2024"\n${include}\n`);
  for (const file of ['src/main.rs', 'README.md', 'LICENSE', 'docs/guide.md',
    'site/node_modules/example/README.md', 'site/node_modules/example/LICENSE',
    '.flatpak-builder/cache/README.md', 'build-flatpak/vendor/LICENSE',
    'scripts/fixtures/example/README.md', 'build-flatpak/vendor/src/unrelated.rs']) {
    mkdirSync(join(dir, file, '..'), { recursive: true });
    writeFileSync(join(dir, file), file === 'src/main.rs' ? 'fn main() {}\n' : 'fixture\n');
  }
  const result = spawnSync('cargo', ['package', '--list', '--offline', '--allow-dirty'], {
    cwd: dir, encoding: 'utf8',
  });
  assert.equal(result.status, 0, result.stderr);
  const files = result.stdout.trim().split('\n');
  for (const file of ['src/main.rs', 'README.md', 'LICENSE', 'docs/guide.md']) assert.ok(files.includes(file), file);
  assert.ok(!files.some(file => /^(site|build-flatpak|scripts|\.flatpak-builder)\//.test(file)), result.stdout);
});
