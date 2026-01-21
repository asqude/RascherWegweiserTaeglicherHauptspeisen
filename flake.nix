{
  description = "RascherWegweiserTaeglicherHauptspeisen - Shows today's Klassiker from the Mensa Academica Aachen";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "RascherWegweiserTaeglicherHauptspeisen";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "CLI tool to display today's Klassiker from Mensa Academica Aachen";
            homepage = "https://github.com/asqude/RascherWegweiserTaeglicherHauptspeisen";
            license = licenses.mit;
            maintainers = [ ];
            mainProgram = "RascherWegweiserTaeglicherHauptspeisen";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
          ];
        };
      }
    );
}
