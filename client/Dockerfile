FROM rust:1.93-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN echo '[workspace]\nmembers = ["protocol", "client"]\nresolver = "3"' > Cargo.toml
COPY Cargo.lock ./

COPY protocol/ ./protocol/
COPY client/ ./client/

RUN cargo build --release --bin client

FROM debian:bookworm-slim as distribution

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false -m -d /app appuser

WORKDIR /app

COPY --from=builder /app/target/release/client /app/client

RUN chown -R appuser:appuser /app

USER appuser

# El binario debería copiarse al host
ENTRYPOINT ["/app/client"]