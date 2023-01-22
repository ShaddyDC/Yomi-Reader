{
  description = "Yomi-Dict is a yomidict dictionary reader";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          buildInputs = with pkgs; [
            pkgconfig
            openssl
            trunk

            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
          ];
        in
        with pkgs;
        {
          devShells.default = stdenv.mkDerivation {
            name = "rust-env";
            nativeBuildInputs = buildInputs;
            RUST_BACKTRACE = 1;
          };
        }
      );
}
