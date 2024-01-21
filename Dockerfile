FROM rust:1.72.0 as builder

WORKDIR /usr/src/app

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.17

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/lecture /usr/local/bin/lecture

CMD ["lecture"]