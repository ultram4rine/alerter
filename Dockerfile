FROM rust:1.59-alpine3.15 as builder

RUN apk add musl-dev openssl-dev pkgconfig

WORKDIR /usr/src/alerter
COPY . ./
RUN cargo build --release

FROM alpine:3.15

RUN addgroup -S alerter && adduser -S alerter -G alerter
USER alerter

WORKDIR /usr/share/alerter
COPY templates ./
COPY --from=builder /usr/src/alerter/target/release/alerter .

EXPOSE 48655/tcp

ENTRYPOINT ["./alerter"]