FROM ekidd/rust-musl-builder:nightly-2020-04-10 AS builder

ADD . ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM alpine:latest
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/duaas \
    /usr/local/bin/

CMD /usr/local/bin/duaas
