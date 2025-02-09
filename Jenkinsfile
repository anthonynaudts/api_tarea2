pipeline {
    agent any

    stages {
        stage('Desplegar en Servidor') {
            steps {
                bat '''
                    ssh -o StrictHostKeyChecking=no root@159.65.162.105 "ls"
                '''
            }
        }
    }
}
