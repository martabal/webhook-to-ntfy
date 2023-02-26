FROM rust:1-alpine3.17 as builder

WORKDIR /app/
COPY Cargo.toml Cargo.lock ./
COPY src /app/src
RUN ls
RUN apk add \
    libressl-dev \
    musl-dev && \
    cargo build --release

FROM alpine:3.17

WORKDIR /app
COPY --from=builder /app/target/release/webhookntfy /app/
RUN apk add s6-overlay

COPY init.sh config.example.yaml /app/
ENV MODE=DOCKER
CMD ["/app/init.sh"]



