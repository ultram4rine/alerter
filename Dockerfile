FROM rust:1.59-slim-bullseye as builder

RUN apt update && \ 
    apt install libssl-dev pkg-config

WORKDIR /usr/src/alerter
COPY . ./
RUN cargo build --release

FROM debian:bullseye-slim

RUN addgroup -S alerter && adduser -S alerter -G alerter
USER alerter

WORKDIR /usr/share/alerter
COPY templates ./
COPY --from=builder /usr/src/alerter/target/release/alerter .

EXPOSE 48655/tcp

ENTRYPOINT ["./alerter"]