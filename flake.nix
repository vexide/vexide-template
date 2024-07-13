{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    cargo-v5.url = "github:vexide/cargo-v5";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, cargo-v5, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        cargo-v5' = cargo-v5.packages.${system}.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            cargo-v5'
            (pkgs.rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" "llvm-tools" "clippy" ];
            })
          ];
        };
      }));
}
