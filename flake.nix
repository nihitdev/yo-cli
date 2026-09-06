{
  description = "yoo project and development environment CLI";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
    in {
      packages = eachSystem (system: {
        default = nixpkgs.legacyPackages.${system}.callPackage ./packaging/nix/package.nix { };
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
