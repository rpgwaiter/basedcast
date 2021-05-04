#!/usr/bin/env groovy
pipeline {
    agent none

    // environment {
    //     PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    // }

    stages {
        stage('Build binaries') {
            agent { dockerfile { 
                filename 'Dockerfile.build' 
                args '-v /mnt/private/builds:/builds'
            }}
            stages {
                stage('Radioscan') {
                    steps {
                        sh '''
                            #!/bin/bash -ex
                            cp settings.toml.example settings.toml
                            cargo build --release --bin radioscan
                            cp target/release/radioscan /builds/radioscan
                        '''
                    }
                }
                stage('Api') {
                    steps {
                        sh '''
                            #!/bin/bash -ex
                            cp settings.toml.example settings.toml
                            cargo build --release --bin basedcast_api
                            cp target/release/basedcast_api /builds/basedcast_api
                        '''
                    }
                }
            }
        }
    }
}
