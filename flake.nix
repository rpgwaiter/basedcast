{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    yarn2nix = { url = "github:nix-community/yarn2nix"; flake = false; };
  };

  outputs = inputs:
    let
      outputs = inputs.nixCargoIntegration.lib.makeOutputs {
        root = ./.;
        buildPlatform = "crate2nix";
        overrides = with pkgs; {
          shell = common: prev: {
            # Packages to be put in $PATH.
            packages = prev.packages ++ [ alsaLib alsaLib.dev];
            # Commands that will be shown in the `menu`. These also get added
            # to packages.
            commands = prev.commands ++ [
              # { package = common.pkgs.git; }
              # { name = "helloworld"; command = "echo 'Hello world'"; }
            ];
            # Environment variables to be exported.
            env = prev.env ++ [
            #   lib.nameValuePair "INCLUDE_PATH" ".:${alsaLib.dev}"
              { name = "INCLUDE_PATH"; eval = ".:${alsaLib.dev}/include/asoundlib.h"; }
            ];
          };
          common = prev: {
            buildInputs = prev.buildInputs ++ [ zlib openssl wasm-pack alsaLib alsaLib.dev  ];

            nativeBuildInputs = prev.nativeBuildInputs ++ [ zlib alsaLib alsaLib.dev ];

            runtimeLibs = prev.runtimeLibs ++ [ zlib alsaLib alsaLib.dev ];
          };
        };
      };
      pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux // rec {
      };
      mkContainer = drv:
        pkgs.dockerTools.buildImage rec {
          name = "${drv.name}";
          config = {
            Cmd = [ "${drv}/bin/${name}" ];
          };
        };
      y2n = (import "${inputs.yarn2nix}/default.nix" { pkgs = pkgs; });
    in with y2n;
    pkgs.lib.recursiveUpdate
    outputs {
      packages.x86_64-linux = {
        basedcast_web = pkgs.yarn2nix-moretea.mkYarnPackage {
            name = "basedcast_web";
            src = ./web;
            packageJSON = ./web/package.json;
            yarnLock = ./web/yarn.lock;
            yarnNix = ./web/yarn.nix;
          };

      } // pkgs.lib.mapAttrs' (name: drv: {
        name = "${name}-container";
        value = mkContainer drv;
      }) outputs.packages.x86_64-linux;

      
    };
      #mapAttrs' mkContainer outputs.packages
}