#!/bin/bash
echo "Construyendo cliente..."
docker build -f client/Dockerfile -t rust-client .

echo "Extrayendo binario..."
docker create --name temp-client rust-client
docker cp temp-client:/app/client ./client
docker rm temp-client

echo "Configurando permisos..."
chmod +x ./client

echo "Cliente instalado"
echo "Ejecuta './client' para iniciar el cliente"