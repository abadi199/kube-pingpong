FROM rust:1.31-stretch as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/src/app/target/release/pong /bin/
EXPOSE 8001
CMD pong