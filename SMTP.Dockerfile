# Use the official Rust image as a base image for building
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Clone the GitHub repository
RUN git clone https://github.com/zkfriendly/relayer-smtp.git

# Change to the directory of the cloned repository
WORKDIR /app/relayer-smtp

# Build the Rust package
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies and setup SSL certificates
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/relayer-smtp/target/release/relayer-smtp /app/relayer-smtp

# Expose port
EXPOSE 3000

# Specify the command to run when the container starts
CMD ["/app/relayer-smtp"]