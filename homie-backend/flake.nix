{
    description = "Rust flake for homie-backend";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
        rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    };

    outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {
                    inherit system overlays;
                };
                rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

                nativeBuildInputs = with pkgs; [
                    rustToolchain
                    pkg-config
                    sqlx-cli
                ];
            in
            {
                devShells.default = pkgs.mkShell {
                    inherit nativeBuildInputs;

                    shellHook = ''
                        export SQLX_OFFLINE="true"
                        # export DATABASE_URL="postgres://user:password@localhost/homie"
                    '';
                };
            }
        );
}
