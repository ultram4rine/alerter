FROM --platform=$BUILDPLATFORM rust:1.66.0-slim-bullseye as builder

RUN USER=root cargo new --bin alerter
WORKDIR /alerter

ENV PKG_CONFIG_ALLOW_CROSS=1

ARG TARGETPLATFORM
RUN echo "Setting variables for ${TARGETPLATFORM:=linux/amd64}" && \
    case "${TARGETPLATFORM}" in \
    linux/amd64) \
    echo "x86_64-unknown-linux-gnu"| tee /tmp/rusttarget; \
    break;; \
    linux/arm64) \
    echo "aarch64-unknown-linux-gnu" | tee /tmp/rusttarget; \
    break;; \
    linux/arm/v7) \
    echo "armv7-unknown-linux-gnueabihf" | tee /tmp/rusttarget; \
    break;; \
    *) echo "unsupported platform ${TARGETPLATFORM}";; \
    esac
RUN rustup target add "$(cat /tmp/rusttarget)"

RUN dpkg --add-architecture arm64 &&\
    dpkg --add-architecture armhf && \
    apt-get update && \ 
    apt-get install -y pkg-config libssl-dev cmake g++ \
    libssl-dev:arm64 gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
    libssl-dev:armhf gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf

COPY .cargo ./.cargo
COPY ./scripts/mangen.rs ./scripts/mangen.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release --target "$(cat /tmp/rusttarget)"
RUN rm src/*.rs
RUN rm ./target/"$(cat /tmp/rusttarget)"/release/deps/alerter*

COPY ./src ./src

RUN cargo build --release --target "$(cat /tmp/rusttarget)"
RUN mv ./target/"$(cat /tmp/rusttarget)"/release/alerter ./target/release/

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