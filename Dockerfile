FROM rust:alpine3.19 as builder
WORKDIR /app
RUN apk add --no-cache musl-dev

COPY . .
RUN cargo build --release

FROM alpine:3.19
COPY --from=builder /app/target/release/rust-learn /usr/local/bin/
CMD ["rust-learn"]
