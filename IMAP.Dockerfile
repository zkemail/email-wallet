# Use the official Rust image as a base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Clone the GitHub repository
RUN git clone https://github.com/zkemail/relayer-imap.git

# Build the Rust package
RUN cargo build

# Specify the command to run when the container starts
CMD ["cargo", "run", "--bin", "smtp-relayer"]