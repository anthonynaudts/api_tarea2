pipeline {
    agent any

    environment {
        REGISTRY_URL = 'http://localhost:8082/repository/docker-hosted'
        REGISTRY_CREDENTIALS = 'nexus-credentials-id'
        IMAGE_NAME = 'anthonynaudts/api_tarea2'
        IMAGE_TAG = "v1"
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
                    bat "docker build -t ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG} ."
                }
            }
        }

        stage('Subir Imagen a Nexus') {
            steps {
                script {
                    withDockerRegistry([credentialsId: REGISTRY_CREDENTIALS, url: "https://${REGISTRY_URL}"]) {
                        bat "docker push %REGISTRY_URL%/%IMAGE_NAME%:%IMAGE_TAG%"
                    }
                }
            }
        }

        stage('Desplegar en Servidor') {
            steps {
                script {
                    sshagent(['server-ssh-key']) {
                        sh """
                        ssh ${SERVER_USER}@${SERVER_IP} '
                        docker pull ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG} &&
                        docker stop ${CONTAINER_NAME} || true &&
                        docker rm ${CONTAINER_NAME} || true &&
                        docker run -d --name ${CONTAINER_NAME} -p ${HOST_PORT}:${CONTAINER_PORT} ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG}
                        '
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
