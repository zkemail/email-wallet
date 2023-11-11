# ------------------ Chef stage -------------------
# Use cargo chef to cache dependencies
FROM rustlang/rust:nightly AS chef

# Install cargo chef
RUN cargo install cargo-chef 

# Work in app
WORKDIR /app

# ------------------ Planner stage -------------------
FROM chef as planner

# Copy files into container
COPY Cargo.toml Cargo.lock ./
COPY packages/relayer ./packages/relayer
COPY packages/utils ./packages/utils
COPY packages/scripts ./packages/scripts

# Create a lockfile for cargo chef
RUN cargo +nightly chef prepare --recipe-path recipe.json

# ------------------ Builder stage -------------------
FROM chef AS builder
WORKDIR /relayer

# Copy over our lock file
COPY --from=planner  /app /relayer

# Build for any AWS machine. Same as cargo build but caches dependencies with the chef to make builds faster.
RUN cargo chef cook --release --recipe-path recipe.json

### Above this all dependencies should be cached as long as our lock file stays the same

# Build binary
RUN cargo build --release

# ------------------ Runtime stage -------------------

# Using super lightweight debian image to reduce overhead
FROM ubuntu:latest AS runtime

# Copy prebuild bin from the Builder stage
COPY --from=builder /relayer/target/release/relayer /relayer/target/release/relayer
COPY --from=builder /relayer/Cargo.toml /relayer/Cargo.toml
COPY --from=builder /relayer/Cargo.lock /relayer/Cargo.lock

EXPOSE 4500

CMD ["relayer/target/release/relayer"]

# This cargo chef logic comes from https://github.com/LukeMathWalker/cargo-chef
# Inspired by Huff: https://github.com/huff-language/huff-rs/blob/main/Dockerfile
