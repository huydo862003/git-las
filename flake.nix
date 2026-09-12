{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust-stable = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-stable;
          rustc = rust-stable;
        };
        git-lase = rustPlatform.buildRustPackage {
          pname = "git-lase";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "-p"
            "git-lase"
          ];
          doCheck = false;
        };
      in
      {
        packages.git-lase = git-lase;
        packages.default = git-lase;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-stable
            cargo-edit
            cargo-watch
            nodejs
            pnpm
          ];
        };
      }
    );
}
