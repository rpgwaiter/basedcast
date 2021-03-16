let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

  with nixpkgs;

  mkShell {
    buildInputs = [
      rustc cargo rustfmt gcc rustup
      pkgconfig
      stdenv.cc.cc
    ];

    shellHook = ''export CFG_DISABLE_CROSS_TESTS=1'';

    LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
    MPD_URL = "127.0.0.1";
    MPD_PORT = "6600";
  }