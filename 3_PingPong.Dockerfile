FROM rust:1.31-stretch as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev
COPY --from=builder /usr/src/app/target/release/pingpong /bin/
EXPOSE 8003
CMD pingpong