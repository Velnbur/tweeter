FROM rust:1.62-slim as builder

ADD ./apis/tweeter-backend /usr/src/apis/tweeter-backend
ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models

WORKDIR /usr/src/apis/tweeter-backend
RUN cargo install --path .

# -----
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/tweeter-backend /usr/local/bin/service
ENTRYPOINT ["service"]