FROM rust:1.62-slim as builder

# copy internal crates
ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models
ADD ./crates/tweeter-auth /usr/src/crates/tweeter-auth
ADD ./crates/tweeter-repos /usr/src/crates/tweeter-repos
ADD ./crates/tweeter-config /usr/src/crates/tweeter-config

# copy service code
ADD ./apis/tweeter-backend /usr/src/apis/tweeter-backend

WORKDIR /usr/src/apis/tweeter-backend
RUN cargo build --release

# -----
FROM debian:buster-slim

COPY --from=builder /usr/src/apis/tweeter-backend/target/release/tweeter-backend /usr/local/bin/service
ENTRYPOINT ["service"]
