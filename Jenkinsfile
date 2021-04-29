#!/usr/bin/env groovy
pipeline {
    agent { label 'master' }

    environment {
        PATH = "/run/wrappers/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin:$PATH" // Needed for NixOS
    }

    stages {
        // Eventually there will be different steps for dev vs live
        stage('Build api') {
            steps {
                echo 'building main.rs'
                sh '''
                    #!/bin/bash -ex
                    direnv allow .
                    eval "$(direnv export bash)"
                    cp .env.example .env
                    cargo build --bin api
                '''
            }
        }
    }
}
