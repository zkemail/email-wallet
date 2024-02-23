# Use a Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /nft_relayer

# Copy the project files into the container
COPY ./nft_relayer .

COPY ./relayer ../relayer

COPY ./utils ../utils

COPY ./circuits ../circuits

COPY ./contracts ../contracts

# Build the project
RUN cargo build --release

# Expose the port
EXPOSE 4500