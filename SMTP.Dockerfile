# Use the official Rust image as a base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Clone the GitHub repository
RUN git clone https://github.com/zkemail/relayer-smtp.git

# Change to the directory of the cloned repository
WORKDIR /app/relayer-smtp

# Build the Rust package
RUN cargo build

# Expose port
EXPOSE 3000

# Specify the command to run when the container starts
CMD ["cargo", "run", "--bin", "relayer-smtp"]