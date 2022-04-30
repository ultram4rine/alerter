FROM rust:1.60.0-slim-bullseye as builder

RUN USER=root cargo new --bin alerter
WORKDIR /alerter

RUN apt-get update && \ 
    apt-get install -y libssl-dev pkg-config cmake build-essential gcc

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./scripts/mangen.rs ./scripts/mangen.rs

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/alerter*
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN adduser --system --group alerter
USER alerter

WORKDIR /usr/share/alerter
COPY templates ./templates
COPY --from=builder /alerter/target/release/alerter .
ENV RUST_LOG="warn"

EXPOSE 3030/tcp

ENTRYPOINT ["./alerter"]