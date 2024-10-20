# Use the base image
FROM sorasue/relayer:latest

# Copy the project files
COPY packages/relayer /relayer/packages/relayer

# Copy the rust-toolchain.toml file
COPY rust-toolchain.toml /relayer/packages/relayer/rust-toolchain.toml

# Set the working directory for the Rust project
WORKDIR /relayer/packages/relayer

# Build the Rust project with caching
RUN cargo build

# Expose port
EXPOSE 4500

# Make sure the script is executable
RUN chmod +x /relayer/packages/relayer/setup_and_run.sh

# Run the script
CMD ["/bin/bash", "/relayer/packages/relayer/setup_and_run.sh"]
