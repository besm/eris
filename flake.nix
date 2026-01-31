{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages = {
          eris = pkgs.rustPlatform.buildRustPackage {
            pname = "eris";
            version = "0.2.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          default = self.packages.${system}.eris;
        };
      }
    );
}
