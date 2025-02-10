pipeline {
    agent any

    environment {
<<<<<<< HEAD
        REGISTRY_URL = 'http://localhost:8082/repository/docker-hosted'
        REGISTRY_CREDENTIALS = 'nexus-credentials-id'
        IMAGE_NAME = 'anthonynaudts/api_tarea2'
        IMAGE_TAG = "v1"
=======
        REGISTRY_URL = 'host.docker.internal:8082'
        REGISTRY_CREDENTIALS = 'nexus-credentials-id'
        IMAGE_NAME = 'anthonynaudts/api_tarea2'
        IMAGE_TAG = 'v1'
>>>>>>> desplegar
        SERVER_USER = 'root'
        SERVER_IP = '159.65.162.105'
        CONTAINER_NAME = 'api_tarea25000'
        CONTAINER_PORT = '8080'
        HOST_PORT = '5000'
    }

<<<<<<< HEAD
=======
    triggers {
        pollSCM('H/5 * * * *')
    }

>>>>>>> desplegar
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
<<<<<<< HEAD
                    sh "docker build -t ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG} ."
=======
                    bat """
                        docker build -t ${IMAGE_NAME}:${IMAGE_TAG} .
                        docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG}
                    """
>>>>>>> desplegar
                }
            }
        }

        stage('Subir Imagen a Nexus') {
            steps {
                script {
                    withDockerRegistry([credentialsId: REGISTRY_CREDENTIALS, url: "http://${REGISTRY_URL}"]) {
<<<<<<< HEAD
                        sh "docker push ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG}"
=======
                        bat "docker push ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG}"
>>>>>>> desplegar
                    }
                }
            }
        }

        stage('Desplegar en Servidor') {
            steps {
                script {
<<<<<<< HEAD
                    sshagent(['server-ssh-key']) {
                        sh """
                        ssh ${SERVER_USER}@${SERVER_IP} '
                        docker pull ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG} &&
                        docker stop ${CONTAINER_NAME} || true &&
                        docker rm ${CONTAINER_NAME} || true &&
                        docker run -d --name ${CONTAINER_NAME} -p ${HOST_PORT}:${CONTAINER_PORT} ${REGISTRY_URL}/${IMAGE_NAME}:${IMAGE_TAG}
                        '
=======
                    withCredentials([sshUserPrivateKey(credentialsId: 'server-ssh-key', keyFileVariable: 'SSH_KEY')]) {
                        bat """
                            icacls "%SSH_KEY%" /inheritance:r
                            icacls "%SSH_KEY%" /grant SYSTEM:F
                            icacls "%SSH_KEY%" /grant "NT AUTHORITY\\SYSTEM:F"
                            icacls "%SSH_KEY%" /grant "Administrators:F"

                            ssh -i "%SSH_KEY%" -o StrictHostKeyChecking=no root@159.65.162.105 "echo 'Conexión Exitosa' && docker pull anthonynaudts/api_tarea2:v1 && docker stop api_tarea25000 || true && docker rm api_tarea25000 || true && docker run -d --name api_tarea25000 -p 5000:8080 anthonynaudts/api_tarea2:v1"
>>>>>>> desplegar
                        """
                    }
                }
            }
        }
<<<<<<< HEAD
=======

        
>>>>>>> desplegar
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
