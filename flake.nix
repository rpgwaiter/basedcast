{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    let
      outputs = inputs.nixCargoIntegration.lib.makeOutputs {
        root = ./.;
        # buildPlatform = "crate2nix";
        # overrides = with inputs.nixpkgs.legacyPackages.x86_64-linux; {
        #   common = prev: {
        #     buildInputs = prev.buildInputs ++ [ zlib openssl wasm-pack ];

        #     nativeBuildInputs = prev.nativeBuildInputs ++ [ zlib ];

        #     runtimeLibs = prev.runtimeLibs ++ [ zlib ];
        #   };
        # };
      };
      pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
      mkContainer = drv:
        pkgs.dockerTools.buildImage rec {
          name = "${drv.name}";
          config = {
            Cmd = [ "${drv}/bin/${name}" ];
          };
        };
    in
    pkgs.lib.recursiveUpdate
    outputs {
      packages.x86_64-linux = pkgs.lib.mapAttrs' (name: drv: {
        name = "${name}-container";
        value = mkContainer drv;
      }) outputs.packages.x86_64-linux;
      devenv = pkgs.dockerTools.buildImage rec {
          name = "devenv";
          # config = {
          #   Cmd = [ "" ];
          # };
        };
    };
      #mapAttrs' mkContainer outputs.packages
}