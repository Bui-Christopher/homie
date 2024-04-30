## Builder
FROM rust:1.77.2-slim as builder

RUN apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY Cargo.toml Cargo.lock ./
COPY homie-data homie-data
COPY homie-core homie-core
COPY homie-api homie-api
COPY homie-webapp homie-webapp

ENV SQLX_OFFLINE true
RUN cargo build --release --bin homie-data

## Runner
FROM rust:1.77.2-alpine3.18 as runner
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder ./target/release/homie-data /app/homie-data
ENTRYPOINT ["./app/homie-data"]
