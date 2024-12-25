FROM rust:alpine as builder

WORKDIR /app/src

RUN apk add pkgconfig openssl-dev libc-dev

RUN apk add --no-cache rustup \
    && rustup toolchain install nightly \
    && rustup default nightly

COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
#RUN apk update \
#    && apk add openssl ca-certificates

COPY --from=builder /app/src/target/release/duaas /app/duaas

ENV ROCKET_ADDRESS 0.0.0.0

CMD ["/app/duaas"]
