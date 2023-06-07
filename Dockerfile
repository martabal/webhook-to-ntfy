FROM rust:1.70-alpine3.18 as builder

COPY src Cargo.toml Cargo.lock /app/

RUN apk add \
    openssl-dev \
    musl-dev && \
    cargo build --release

FROM alpine:3.18

WORKDIR /app

COPY --from=builder /app/target/release/webhookntfy /app/
COPY init.sh config.example.yaml /app/

CMD ["/app/init.sh"]