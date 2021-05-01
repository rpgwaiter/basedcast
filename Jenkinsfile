#!/usr/bin/env groovy
pipeline {
    agent { label 'master' }

    environment {
        PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    }

    stages {
        // Eventually there will be different steps for dev vs live
        stage('Build radioscan') {
            agent {
                docker { image 'nixpkgs/nix-flakes' }
            }
            steps {
                echo 'building radioscan'
                sh '''
                    pwd
                    ls -Alh
                    direnv allow .
                    eval "$(direnv export bash)"
                    cp settings.toml.example settings.toml
                    nix build .
                '''
            }
        }
    }
}
