# Use Debian Bullseye as base image with Node.js 18
FROM node:18-bullseye AS contract-builder

# Install required tools and dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

# Set the working directory
WORKDIR /app

# Copy package files
COPY package*.json yarn.lock ./
COPY packages/contracts/package*.json ./packages/contracts/

# Install dependencies
RUN yarn install
RUN cd packages/contracts && yarn install

# Copy the contracts directory
COPY packages/contracts /app/packages/contracts

# Build the contracts
RUN cd packages/contracts && forge build --skip tests --skip scripts

# Use the latest official Rust image as the base for the next stage
FROM rust:latest AS rust-builder

# Set the working directory
WORKDIR /app

# Copy the built contracts/artifacts from the previous stage
COPY --from=contract-builder /app/packages/contracts/artifacts /app/packages/contracts/artifacts

# Copy the packages/relayer directory
COPY packages/relayer /app/packages/relayer

# Set the working directory for relayer
WORKDIR /app/packages/relayer

RUN cargo build --release

# for the final stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies and setup SSL certificates
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

# Set the working directory
WORKDIR /app

# Copy only the necessary target directory from the rust-builder stage
COPY --from=rust-builder /app/packages/relayer/target/release/relayer /app/relayer

# Copy the necessary directories from packages/relayer in the final stage
COPY --from=rust-builder /app/packages/relayer/eml_templates /app/eml_templates
COPY --from=rust-builder /app/packages/relayer/graphql /app/graphql
COPY --from=rust-builder /app/packages/relayer/input_files /app/input_files

CMD [ "/app/relayer" ]