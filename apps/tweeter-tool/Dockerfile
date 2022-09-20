FROM rust:1.62-slim as builder

# copy internal crates
ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models
ADD ./crates/tweeter-auth /usr/src/crates/tweeter-auth

# copy service code
ADD ./apps/tweeter-tool /usr/src/apps/tweeter-tool

WORKDIR /usr/src/apps/tweeter-tool
RUN cargo build --release

# -----
FROM debian:buster-slim

COPY --from=builder /usr/src/apps/tweeter-tool/target/release/tweeter-tool /usr/local/bin/tweeter-tool
ENTRYPOINT ["tool"]
