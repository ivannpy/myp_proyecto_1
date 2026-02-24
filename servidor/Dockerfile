FROM rust:1.93

WORKDIR /usr/src/servidor
COPY . .

RUN cargo install --path .

CMD ["servidor"]
