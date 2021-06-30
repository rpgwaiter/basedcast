{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixCargoIntegration = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell.url = "github:numtide/devshell";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    let
      outputs = inputs.nixCargoIntegration.lib.makeOutputs rec {
        root = ./.;
        buildPlatform = "crate2nix";
        overrides = with pkgs; {
          shell = common: prev: {
            commands = 
              # { package = common.pkgs.git; }
              let
                devRoot = "/home/robots/dev";
                mpdRoot = "${devRoot}/mpd";
                icecastRoot = "${devRoot}/icecast";
                icecastConf = pkgs.writeText "icecast.xml" ''
                  <icecast>
                  <hostname>127.0.0.1</hostname>
                  <location>Localhost</location>

                  <authentication>
                    <admin-user>radio</admin-user>
                    <admin-password>radio</admin-password>
                  </authentication>

                  <paths>
                    <logdir>${icecastRoot}/logs</logdir>
                    <adminroot>${pkgs.icecast}/share/icecast/admin</adminroot>
                    <webroot>${pkgs.icecast}/share/icecast/web</webroot>
                    <alias source="/" dest="/status.xsl"/>
                  </paths>

                  <listen-socket>
                    <port>9111</port>
                    <bind-address>127.0.0.1</bind-address>
                  </listen-socket>

                  <authentication>
                  <source-password>radio</source-password>
                </authentication>

                </icecast>

                '';
                mpdConf = pkgs.writeText "basedcast-dev-mpd.conf" ''
                  music_directory     "/mnt/public/Radio/radiofiles"
                  playlist_directory  "${mpdRoot}/playlists"
                  db_file             "${mpdRoot}/tag_cache"

                  state_file          "${mpdRoot}/state"
                  sticker_file        "${mpdRoot}/sticker.sql"

                  bind_to_address "127.0.0.1"

                  # password "radio@read,add,control,admin"

                  audio_output {
                    mixer_type "software"
                    type "shout"
                    #always_on "yes"
                    encoder "vorbis" # fix
                    format "44100:16:2"
                    bitrate "192"
                    name "BasedCast"
                    host "127.0.0.1"
                    port "9111"
                    mount "/vgm.ogg"
                    user "source"
                    password "radio"
                  }
                  audio_output {
                    type "null"
                    name "fake out"
                  }
                '';
              in prev.commands ++ [
              rec { 
                name = "dev";
                command = ''
                  export PGHOST=$HOME/postgres
                  export PGDATA=$PGHOST/data
                  export PGDATABASE=postgres
                  export PGLOG=$PGHOST/postgres.log
                  echo 'Killing any existing mpd/icecast'
                  ${pkgs.killall}/bin/killall mpd 2> /dev/null # Probably could use some work
                  ${pkgs.killall}/bin/killall icecast 2> /dev/null
                  sleep 1

                  run_dev() {
                    mkdir -p $PGHOST
                    mkdir -p ${mpdRoot}/{playlists, tag_cache}
                    mkdir -p ${icecastRoot}/logs

                    if [ ! -d $PGDATA ]; then
                      initdb --auth=trust --no-locale --encoding=UTF8
                    fi

                    if ! pg_ctl status
                    then
                      pg_ctl start -l $PGLOG -o "--unix_socket_directories='$PGHOST'"
                    fi
                    
                    ${pkgs.icecast}/bin/icecast -b -c ${icecastConf}
                    echo 'Started Icecast!'
                    sleep .5
                    ${pkgs.mpd}/bin/mpd ${mpdConf}
                    echo 'Started MPD!'
                    sleep .5
                    ${pkgs.mpc_cli}/bin/mpc clear
                    ${pkgs.mpc_cli}/bin/mpc ls |
                    ${pkgs.mpc_cli}/bin/mpc add
                    ${pkgs.mpc_cli}/bin/mpc play
                    ${outputs.packages.x86_64-linux.radioscan}/bin/radioscan
                  }

                  trap 'run_dev' EXIT
                '';
                # ${outputs.packages.x86_64-linux.radioscan}/bin/radioscan
            # env = prev.env ++ [];
              }];
          };
          common = prev: {
            buildInputs = prev.buildInputs ++ [ ];
            nativeBuildInputs = prev.nativeBuildInputs ++ [ ];
            runtimeLibs = prev.runtimeLibs ++ [ ];
          };
        };
      };
      pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux // rec {
      };
      mkContainer = drv:
        pkgs.dockerTools.buildImage rec {
          name = "${drv.name}";
          config.Cmd = [ "${drv}/bin/${name}" ];
        };
      
      
    in
    pkgs.lib.recursiveUpdate
    outputs {
      packages.x86_64-linux = pkgs.lib.mapAttrs' (name: drv: {
        name = "${name}-container";
        value = mkContainer drv;
      }) outputs.packages.x86_64-linux;
    };
    
}