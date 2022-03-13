FROM rust:1.59-slim-bullseye as builder

RUN USER=root cargo new --bin alerter
WORKDIR /alerter

RUN apt-get update && \ 
    apt-get install -y libssl-dev pkg-config cmake build-essential gcc

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/alerter*
RUN cargo build --release

FROM debian:bullseye-slim

RUN adduser --system --group alerter
USER alerter

WORKDIR /usr/share/alerter
COPY templates ./
COPY --from=builder /alerter/target/release/alerter .

EXPOSE 48655/tcp

ENTRYPOINT ["./alerter"]