{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";

      # Get a specific rust version
      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust = (mozilla.rustChannelOf {
        date = "2020-01-01"; # get the current date with `date -I`
        channel = "nightly";
        sha256 = "sha256-dzN3TB8+Q1YGKntiWSFZqml0xWPHQPdScV0zGVjvRmk=";
      }).rust;

      # Override the version used in naersk
      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
    in rec {
      # `nix build`
      packages.basedcast = naersk-lib.buildPackage {
        pname = "basedcast";
        root = ./.;
      };
      defaultPackage = packages.basedcast;

      # `nix run`
      apps.basedcast = utils.lib.mkApp {
        drv = packages.basedcast;
      };
      defaultApp = apps.basedcast;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs;[
          rust postgresql.lib
          pkg-config
          diesel-cli
          ffmpeg
          llvm
          llvmPackages.libclang
          stdenv.cc.libc
          clang ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
      };
    });
}
