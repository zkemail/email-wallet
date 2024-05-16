# syntax=docker/dockerfile:1.6

FROM rust:latest

# Remove unnecessary apt clean configuration
RUN rm -f /etc/apt/apt.conf.d/docker-clean

# Update and upgrade packages with caching
RUN --mount=type=cache,target=/var/cache/apt \
    apt update && apt upgrade -y \
    && rm -rf /var/lib/apt/lists/*

# Use bash as the shell
SHELL ["/bin/bash", "-c"]

# Install NVM, Node.js, and Yarn
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash \
    && . /root/.nvm/nvm.sh \
    && . /root/.nvm/bash_completion \
    && bash -i -c "nvm install 18 && nvm use 18" \
    && bash -i -c "npm install -g yarn"

WORKDIR /relayer

# Copy project files
COPY . .

# Install Yarn dependencies with caching
RUN --mount=type=cache,target=/var/cache/yarn \
    bash -i -c "yarn" \
    && rm -rf /var/lib/yarn/lists/*

# Install Foundry and build contracts
RUN curl -L https://foundry.paradigm.xyz | bash \
    && bash -i -c "foundryup"

WORKDIR /relayer/packages/contracts
RUN bash -i -c "forge build"

WORKDIR /relayer/packages/relayer

# Build Rust project with caching
RUN --mount=type=cache,target=/var/cache/cargo \
    cargo build \
    && rm -rf /var/lib/cargo/lists/*

# Expose port
EXPOSE 4500

# Set the default command
CMD ["cargo", "run"]