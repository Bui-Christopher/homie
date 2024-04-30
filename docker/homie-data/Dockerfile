## Builder
FROM rust:1.77.2-slim as builder

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY Cargo.toml Cargo.lock ./
COPY homie-data homie-data
COPY homie-core homie-core
COPY homie-api homie-api
COPY homie-webapp homie-webapp
COPY .sqlx .sqlx

ENV SQLX_OFFLINE true
RUN cargo build --release --bin homie-data --target x86_64-unknown-linux-musl 

## Runner
FROM rust:1.77.2-alpine3.18 as runner
RUN apk add --no-cache bash
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder ./target/x86_64-unknown-linux-musl/release//homie-data homie-data
# ENTRYPOINT ["./homie-data"]
CMD /bin/bash
