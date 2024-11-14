{
    description = "Rust flake for homie-frontend";

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

                buildInputs = with pkgs; [
                    cacert
                    openssl
                    trunk
                ];
                nativeBuildInputs = with pkgs; [
                    pkg-config

                    leptosfmt
                    rustToolchain
                    sqlx-cli
                ];
            in
            {
                devShells.default = pkgs.mkShell {
                    inherit buildInputs nativeBuildInputs;

                    shellHook = ''
                        export SQLX_OFFLINE="true"
                        # export DATABASE_URL="postgres://user:password@localhost/homie"
                    '';
                };
            }
        );
}
