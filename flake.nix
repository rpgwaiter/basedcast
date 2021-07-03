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
      pink = "tput setaf 2 ";
      green_bg = "tput setab 2";
      reset = "tput sgr0";
      devEcho = "${pink}; echo -n '[DEV] ';${reset}; echo ";
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

                  ${devEcho} "Killing any existing mpd/icecast"
                  pgrep mpd | xargs kill > /dev/null 2>&1 && 
                  pgrep icecast | xargs kill > /dev/null 2>&1 &&
                  sleep 1

                  run_dev() {
                    ${devEcho} "Creating state directories.."
                    mkdir -p $PGHOST
                    mkdir -p '${mpdRoot}/playlists' '${icecastRoot}/logs'

                    if [ ! -d $PGDATA ]; then
                      ${devEcho} "No postgres data found, creating database..."
                      initdb --auth=trust --no-locale --encoding=UTF8
                    fi

                    if ! pg_ctl status
                    then
                      ${devEcho} "Starting postgres"
                      pg_ctl start -l $PGLOG -o "--unix_socket_directories='$PGHOST'"
                    fi

                    ${devEcho} "Ensuring /radio database exists"
                    createdb radio > /dev/null &&

                    ${pkgs.diesel-cli}/bin/diesel migration run --database-url=postgres://localhost/radio
                    ${pkgs.diesel-cli}/bin/diesel migration redo --database-url=postgres://localhost/radio
                    
                    ${devEcho} "Starting Icecast.." > /dev/null
                    ${pkgs.icecast}/bin/icecast -b -c ${icecastConf}
                    
                    sleep .5
                    ${devEcho} "Starting MPD.."
                    ${pkgs.mpd}/bin/mpd ${mpdConf}
                    
                    sleep .5
                    ${devEcho} "Populating MPD's playlist.."
                    ${pkgs.mpc_cli}/bin/mpc clear
                    ${pkgs.mpc_cli}/bin/mpc listall |
                    ${pkgs.mpc_cli}/bin/mpc add
                    ${devEcho} "Starting audio stream.."
                    ${pkgs.mpc_cli}/bin/mpc play
                    ${outputs.packages.x86_64-linux.radioscan}/bin/radioscan
                    ${outputs.packages.x86_64-linux.basedcast_api}/bin/basedcast_api
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