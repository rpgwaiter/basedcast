#!/usr/bin/env groovy
pipeline {
    agent {
        docker { 
            image 'nixpkgs/nix-flakes' 
        }
    }

    // environment {
    //     PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    // }

    stages {
        // Eventually there will be different steps for dev vs live
        stage('Build radioscan') {
            steps {
                echo 'building radioscan'
                sh '''
                    #!/bin/bash -ex
                    nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs
                    nix-channel --update
                    cp settings.toml.example settings.toml
                    nix build .#radioscan
                '''
            }
        }
    }
}
