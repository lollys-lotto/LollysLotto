FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN apt-get update && \
  apt-get install -y build-essential clang libudev-dev pkg-config

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ../ ./
RUN cargo build --release --bin lollys_lotto_event_monitor

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:22.04 AS runtime

RUN apt-get update && \
  apt-get install -y ca-certificates && \
  apt-get clean

COPY --from=builder /app/target/release/lollys_lotto_event_monitor /usr/local/bin/lollys_lotto_event_monitor
