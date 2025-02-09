pipeline {
    agent any

    environment {
        REGISTRY_URL = 'host.docker.internal:8082'
        REPOSITORY_NAME = 'anthonynaudts/api_tarea2'
        IMAGE_TAG = 'v1'
        IMAGE_NAME = "${REGISTRY_URL}/${REPOSITORY_NAME}:${IMAGE_TAG}"
        REGISTRY_CREDENTIALS = 'nexus-credentials-id'
        SERVER_USER = 'root'
        SERVER_IP = '159.65.162.105'
        CONTAINER_NAME = 'api_tarea25000'
        CONTAINER_PORT = '8080'
        HOST_PORT = '5000'
    }

    stages {
        stage('Verificar Rama') {
            steps {
                script {
                    if (env.BRANCH_NAME == 'main' || env.BRANCH_NAME == 'develop') {
                        error "El despliegue en ${env.BRANCH_NAME} solo se permite por Pull Request."
                    }
                }
            }
        }

        stage('Checkout Código') {
            steps {
                checkout scm
            }
        }

        stage('Construir Imagen Docker') {
            steps {
                script {
                    bat "docker build -t ${IMAGE_NAME} ."
                }
            }
        }

        stage('Subir Imagen a Nexus') {
            steps {
                script {
                    withDockerRegistry([credentialsId: REGISTRY_CREDENTIALS, url: "http://${REGISTRY_URL}/repository/docker-hosted"]) {
                        bat """
                            docker push ${IMAGE_NAME}
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
