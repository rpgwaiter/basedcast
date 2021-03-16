let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

  with nixpkgs;

  mkShell {
    buildInputs = [
      rustc cargo rustfmt gcc
    ];

    shellHook = ''export CFG_DISABLE_CROSS_TESTS=1'';

    MPD_URL = "127.0.0.1";
    MPD_PORT = "6600";
  }