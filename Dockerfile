FROM rust:1.62-slim as builder
WORKDIR /usr/src/rust-api-example
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/rust-api-example /usr/local/bin/service
CMD ["service"]