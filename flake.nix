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
      api_url = "http://localhost:8000";
      stream_url = "http://cast.based.radio/vgm.ogg";
      outputs = inputs.nixCargoIntegration.lib.makeOutputs {
        root = ./.;
        buildPlatform = "crate2nix";
        overrides = with pkgs; {
          shell = common: prev: {
            env = prev.env ++ [
              { name = "REACT_APP_API_URL"; eval = api_url; }
              { name = "REACT_APP_STREAM_URL"; eval = stream_url; }
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
        basedcast_web = pkgs.yarn2nix.mkYarnPackage {
            name = "basedcast_web";
            src = ./web;
            packageJSON = ./web/package.json;
            yarnLock = ./web/yarn.lock;
            yarnNix = ./web/yarn.nix;
        } // pkgs.lib.mapAttrs' (name: drv: {
          name = "${name}-container";
          value = mkContainer drv;
        }) outputs.packages.x86_64-linux;
      };
    };
      #mapAttrs' mkContainer outputs.packages
}