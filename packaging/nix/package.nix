{ lib, rustPlatform, installShellFiles }:
let
  manifest = builtins.fromTOML (builtins.readFile ../../Cargo.toml);
in
rustPlatform.buildRustPackage {
  pname = manifest.package.name;
  version = manifest.package.version;
  src = lib.fileset.toSource {
    root = ../..;
    fileset = lib.fileset.unions [
      ../../Cargo.toml ../../Cargo.lock ../../src ../../tests
      ../../README.md ../../LICENSE
    ];
  };
  cargoLock.lockFile = ../../Cargo.lock;
  nativeBuildInputs = [ installShellFiles ];
  postInstall = ''
    installShellCompletion --cmd yoo \
      --bash <($out/bin/yoo completions bash) \
      --zsh <($out/bin/yoo completions zsh) \
      --fish <($out/bin/yoo completions fish)
    install -Dm644 LICENSE $out/share/licenses/yoo/LICENSE
    install -Dm644 README.md $out/share/doc/yoo/README.md
  '';
  meta = {
    description = manifest.package.description;
    homepage = manifest.package.repository;
    license = lib.licenses.gpl3Plus;
    mainProgram = "yoo";
    platforms = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
  };
}
