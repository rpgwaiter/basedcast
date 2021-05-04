#!/usr/bin/env groovy
pipeline {
    agent none

    environment {
        //PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
        registry = 'rpgwaiter/basedcast'
        registryCredential = 'dockerhub'
        dockerImage = ''
        
    }

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
                            parallel cp -v settings.toml.example ::: /builds/settings.toml /builds/settings.toml
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
        stage('Containers') {
            stages {
                stage('Create Radioscan') {
                    steps {
                        script {
                            docker.build(registry + ":$BUILD_NUMBER", "radioscan/Dockerfile")
                        }
                    }
                }
            }
        }
    }
}
