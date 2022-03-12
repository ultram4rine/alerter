FROM rust:1.59-slim-bullseye as builder

RUN apt-get update && \ 
    apt-get install -y libssl-dev pkg-config cmake build-essential gcc

WORKDIR /usr/src/alerter
COPY . ./
RUN cargo build --release

FROM debian:bullseye-slim

RUN adduser --system --group alerter
USER alerter

WORKDIR /usr/share/alerter
COPY templates ./
COPY --from=builder /usr/src/alerter/target/release/alerter .

EXPOSE 48655/tcp

ENTRYPOINT ["./alerter"]