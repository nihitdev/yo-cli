{
  description = "yoo project and development environment CLI";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
    in {
      packages = eachSystem (system: {
        default = (nixpkgs.legacyPackages.${system}.callPackage ./packaging/nix/package.nix { }).overrideAttrs (old: {
          # The Rust test suite exercises Git repository detection, so keep Git
          # available in the isolated Nix build environment.
          nativeBuildInputs = (old.nativeBuildInputs or [ ]) ++ [ nixpkgs.legacyPackages.${system}.git ];
        });
      });
      apps = eachSystem (system: {
        default = {
          type = "app";
          meta.description = "Project and development environment information";
          program = "${self.packages.${system}.default}/bin/yoo";
        };
      });
      checks = eachSystem (system: {
        build = self.packages.${system}.default;
      });
    };
}
