FROM rust:1.62-slim as builder

ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models

ADD ./apps/tweeter-frontend /usr/src/apps/tweeter-frontend

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/apps/tweeter-frontend

RUN trunk build --release

# -----
FROM debian:buster-slim
COPY --from=builder /usr/src/apps/tweeter-frontend/dist /app

EXPOSE 8080

WORKDIR /app
CMD ["trunk", "serve", "--release", "--address", "0.0.0.0"]
