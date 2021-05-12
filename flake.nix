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
          common = prev: {
            buildInputs = prev.buildInputs ++ [ zlib openssl wasm-pack ];

            nativeBuildInputs = prev.nativeBuildInputs ++ [ zlib ];

            runtimeLibs = prev.runtimeLibs ++ [ zlib ];
          };
        };
      };
      pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux // rec {
        nodePackages.create-react-app = inputs.nixpkgs.legacyPackages.x86_64-linux.nodePackages.create-react-app.override {
          preRebuild = ''
            substituteInPlace $(find -type f -name createReactApp.js) \
                --replace "path.join(root, 'yarn.lock')" "path.join(root, 'yarn.lock')); fs.chmodSync(path.join(root, 'yarn.lock'), 0o644"
          '';
        };
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