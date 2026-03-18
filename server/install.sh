#!/bin/bash

# Script de despliegue para el servidor en producción
# Uso: ./deploy.sh [puerto]

set -e

PORT=${1:-1234}
CONTAINER_NAME="rust-server-prod"
IMAGE_NAME=${2:-"ivannpy/rust-server:latest"}

# Limpiar contenedor existente si existe alguno
docker stop $CONTAINER_NAME 2>/dev/null || true
docker rm $CONTAINER_NAME 2>/dev/null || true

# Pull imagen desde registry
docker pull $IMAGE_NAME

# Ejecutar servidor
docker run -d \
  --name $CONTAINER_NAME \
  --restart unless-stopped \
  -p $PORT:$PORT \
  -e SERVER_PORT=$PORT \
  $IMAGE_NAME

echo "Servidor desplegado en puerto $PORT"
echo "Contenedor: $CONTAINER_NAME"