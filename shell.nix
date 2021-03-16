let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

  with nixpkgs;

  mkShell {
    buildInputs = [
      latest.rustChannels.stable.rust
      rls
    ];

    MPD_URL = "127.0.0.1";
    MPD_PORT = "6600";
  }