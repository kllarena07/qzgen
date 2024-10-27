FROM rust:1.80-alpine3.20 AS builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/qzgen
COPY . .
RUN cargo build --release

FROM alpine:3.20.0
COPY --from=builder /usr/src/qzgen/target/release/qzgen /
CMD ["./qzgen"]

