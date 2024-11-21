## Builder
FROM rust:1.81.0-slim AS builder

WORKDIR /src
RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev curl && \
    update-ca-certificates

COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin homie-data --target x86_64-unknown-linux-musl

## Runner
FROM scratch AS runner
## Copy binary and certs from builder
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder ./src/target/x86_64-unknown-linux-musl/release/homie-data homie-data
ENTRYPOINT ["./homie-data"]
