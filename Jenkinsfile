#!/usr/bin/env groovy
pipeline {
    agent {
        dockerfile { 
            dir 'radioscan'
            filename 'Dockerfile.build' 
            args '-u 0:0'
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
                    nix build .#radioscan
                '''
            }
        }
    }
}
