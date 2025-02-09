pipeline {
    agent any

    environment {
        REGISTRY_URL = 'host.docker.internal:8082'
        IMAGE_NAME = 'anthonynaudts/api_tarea2'
        IMAGE_TAG = 'v1'
        SERVER_USER = 'root'
        SERVER_IP = '159.65.162.105'
        CONTAINER_NAME = 'api_tarea25000'
        CONTAINER_PORT = '8080'
        HOST_PORT = '5000'
    }

    stages {
        

        stage('Desplegar en Servidor') {
    steps {
        script {
            withCredentials([sshUserPrivateKey(credentialsId: 'server-ssh-key', keyFileVariable: 'SSH_KEY')]) {
                bat """
                ssh -vvv -o StrictHostKeyChecking=no -i %SSH_KEY% ${SERVER_USER}@${SERVER_IP} ^
                "echo 'Conexión Exitosa desde Jenkins'"
                """
            }
        }
    }
}
    }

    post {
        success {
            echo "Despliegue exitoso"
        }
        failure {
            echo "Despliegue fallido"
        }
    }
}
