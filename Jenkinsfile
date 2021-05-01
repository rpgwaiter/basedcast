#!/usr/bin/env groovy
pipeline {
    agent {
        docker { 
            image 'nixpkgs/nix-flakes' 
            args '-v $HOME:/root/basedcast'
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
                    nix-env -iA nixos.nix-direnv
                    direnv allow .
                    eval "$(direnv export bash)"
                    cp settings.toml.example settings.toml
                    nix build .
                '''
            }
        }
    }
}
