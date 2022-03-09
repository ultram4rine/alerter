FROM rust:1.59-alpine3.15 as builder

RUN apk add musl-dev openssl-dev pkgconfig

WORKDIR /usr/src/alerter
ADD . ./
RUN cargo build --release

FROM alpine:3.15

COPY templates /etc/alerter/
COPY --from=builder /usr/src/alerter/target/release/alerter /usr/local/bin/alerter

EXPOSE 48655/tcp

CMD ["/usr/local/bin/alerter"]