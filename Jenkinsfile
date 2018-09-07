pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh './configure'
        sh 'cargo build --all-features'
      }
    }
    stage('Test') {
      parallel {
        stage('cargo') {
          steps {
            sh 'cargo test'
          }
        }

        stage('docker') {
          steps {
            sh './tests/run.sh'
          }
        }
      }
    }
    stage('Style checks') {
      parallel {
        stage('clippy') {
          steps {
            sh 'cargo clippy -- -Dwarnings'
          }
        }
        stage('rustfmt') {
          steps {
            sh 'cargo fmt --all -- --check'
          }
        }
      }
    }
  }
}
