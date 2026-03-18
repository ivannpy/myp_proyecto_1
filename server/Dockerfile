# Contrucción del binario

FROM rust:1.93-slim as builder

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Directorio para contrucción
WORKDIR /app

# Copiamos archivos de dependencias y versiones al contenedor
RUN echo '[workspace]\nmembers = ["protocol", "server"]\nresolver = "3"' > Cargo.toml
COPY Cargo.lock ./

# Copiamos código fuente al contenedor
COPY protocol/ ./protocol/
COPY server/ ./server/

# Contruimos el binario en el contenedor en modo release
RUN cargo build --release --bin server

# Ejecución sobre Debian solo con lo necesario
FROM debian:bookworm-slim

# Instalar dependencias para debian
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario del sistema, con shell por defecto,
# con directorio home dado por /app
RUN useradd -r -s /bin/false -m -d /app appuser

# Nos movemos al directorio de trabajo
WORKDIR /app

# Copiamos el binario compilado
COPY --from=builder /app/target/release/server /app/server

# /app es propiedad de appuser
RUN chown -R appuser:appuser /app

# Cambiamos de usuario
USER appuser

EXPOSE 1234

# Chequeo de salud del contenedor
HEALTHCHECK --interval=160s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${SERVER_PORT:1234}/health || exit 1

# Ejecutar el binario
CMD ["sh", "-c", "./server ${SERVER_PORT:-1234}"]