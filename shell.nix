# Latest Nightly
with import <nixpkgs> {};
let src = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation rec {
  name = "rust-env";
  buildInputs = [
    # Note: to use stable, just replace `nightly` with `stable`
    latest.rustChannels.nightly.rust

    # Add some extra dependencies from `pkgs`
    pkg-config openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}