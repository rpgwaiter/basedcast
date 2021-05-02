#!/usr/bin/env groovy
pipeline {
    agent {
        docker { 
            image 'rustlang/rust:nightly'
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
                    cp settings.toml.example settings.toml
                    cargo build --release radioscan
                '''
            }
        }
    }
}
