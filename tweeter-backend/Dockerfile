FROM rust:1.62-slim as builder

ADD ./tweeter-backend /usr/src/tweeter-backend
ADD ./tweeter-schemas /usr/src/tweeter-schemas
ADD ./tweeter-models /usr/src/tweeter-models

WORKDIR /usr/src/tweeter-backend
RUN cargo install --path .

# -----
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/tweeter-backend /usr/local/bin/service
ENTRYPOINT ["service"]
