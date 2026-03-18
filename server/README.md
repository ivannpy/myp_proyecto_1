# Construcción y ejecución local del servidor

__Construir la imagen del servidor localmente__
docker build -f server/Dockerfile -t rust-server .

__Ejecutar el servidor localmente en puerto 1234__
docker run -d --name rust-server -p 1234:1234 -e SERVER_PORT=1234 rust-server

__Verificar que esté corriendo__
docker ps
docker logs rust-server

# Construcción de la imagen del servidor en DockerHub

__Construir la imagen y subirla a DockerHub__

docker build -f server/Dockerfile -t ivannpy/rust-server:latest .
docker push ivannpy/rust-server:latest

__Ejecutarla desde el servidor__

deploy.sh 1234