FROM rust:buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/webhookntfy /app/
COPY init.sh config.example.yaml /app/

CMD ["/app/init.sh"]
