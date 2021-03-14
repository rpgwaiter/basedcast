#!/usr/bin/env groovy
pipeline {
    agent { label 'master' }

    environment {
        PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    }

    stages {
        stage('Build') {
            steps {
                echo 'building main.rs'
                sh '''
                    #!/bin/bash -ex
                    direnv allow .
                    eval "$(direnv export bash)"
                    cargo build
                '''
            }
        }
    }
}
