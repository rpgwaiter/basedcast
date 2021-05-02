#!/usr/bin/env groovy
pipeline {
    agent none

    // environment {
    //     PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    // }

    stages {
        stage('Build radioscan') {
            agent {
                dockerfile { 
                    dir 'radioscan'
                    filename 'Dockerfile.build'
                }
            }
            steps {
                echo 'building radioscan'
                sh '''
                    #!/bin/bash -ex
                    cp settings.toml.example settings.toml
                    cargo build --release --bin radioscan
                '''
            }
        }
        stage('Build api') {
            agent {
                dockerfile { 
                    dir '.'
                    filename 'Dockerfile.build'
                }
            }
            steps {
                echo 'building api'
                sh '''
                    #!/bin/bash -ex
                    cp settings.toml.example settings.toml
                    cargo build --release --bin basedcast_api
                '''
            }
        }
    }
}
