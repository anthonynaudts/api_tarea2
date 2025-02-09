pipeline {
    agent any

    environment {
        REGISTRY_URL = 'localhost:8081'
        REGISTRY_CREDENTIALS = 'nexus-credentials-id'
        IMAGE_NAME = 'anthonynaudts-api_tarea2'
        IMAGE_TAG = "v1"
        DOCKER_REPO = "${REGISTRY_URL}/repository/docker-hosted"
        SERVER_USER = 'root'
        SERVER_IP = '159.65.162.105'
        SSH_CREDENTIALS_ID = 'server-ssh-key'
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
                    echo "Construyendo imagen Docker..."
                    bat "docker build -t ${IMAGE_NAME}:${IMAGE_TAG} ."
                }
            }
        }

        stage('Subir Imagen a Nexus') {
            steps {
                script {
                    echo "Iniciando autenticación en Nexus..."
                    withDockerRegistry([credentialsId: REGISTRY_CREDENTIALS, url: "http://${REGISTRY_URL}"]) {
                        bat "docker login -u admin -p Starlink1208!! http://${REGISTRY_URL}"

                        echo "Etiquetando imagen para Nexus..."
                        bat "docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${DOCKER_REPO}/${IMAGE_NAME}:${IMAGE_TAG}"

                        echo "Enviando imagen a Nexus..."
                        bat "docker push ${DOCKER_REPO}/${IMAGE_NAME}:${IMAGE_TAG}"
                    }
                }
            }
        }

        stage('Desplegar en Servidor') {
            steps {
                script {
                    echo "Desplegando en el servidor..."
                    sshagent([SSH_CREDENTIALS_ID]) {
                        bat """
                        ssh -o StrictHostKeyChecking=no ${SERVER_USER}@${SERVER_IP} "
                        docker login -u admin -p Starlink1208!! http://${REGISTRY_URL} &&
                        docker pull ${DOCKER_REPO}/${IMAGE_NAME}:${IMAGE_TAG} &&
                        docker stop ${IMAGE_NAME} || true &&
                        docker rm ${IMAGE_NAME} || true &&
                        docker run -d --name ${IMAGE_NAME} -p 5000:8080 ${DOCKER_REPO}/${IMAGE_NAME}:${IMAGE_TAG}
                        "
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
