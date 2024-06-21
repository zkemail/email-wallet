# Use the latest official Rust image as the base
FROM rust:latest

# Use bash as the shell
SHELL ["/bin/bash", "-c"]

# Install NVM, Node.js, and Yarn
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash \
    && . $HOME/.nvm/nvm.sh \
    && nvm install 18 \
    && nvm alias default 18 \
    && nvm use default \
    && npm install -g yarn

# Set the working directory
WORKDIR /relayer

# Pre-configure Git to avoid common issues and increase clone verbosity
RUN git config --global advice.detachedHead false \
    && git config --global core.compression 0 \
    && git config --global protocol.version 2 \
    && git config --global http.postBuffer 1048576000 \
    && git config --global fetch.verbose true

# Copy project files
COPY . .

# Remove the packages/relayer directory
RUN rm -rf packages/relayer

# Install Yarn dependencies with retry mechanism
RUN . $HOME/.nvm/nvm.sh && nvm use default && yarn || \
    (sleep 5 && yarn) || \
    (sleep 10 && yarn)

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash \
    && source $HOME/.bashrc \
    && foundryup

# Verify Foundry installation
RUN source $HOME/.bashrc && forge --version

# Set the working directory for contracts
WORKDIR /relayer/packages/contracts

# Install Yarn dependencies for contracts
RUN source $HOME/.nvm/nvm.sh && nvm use default && yarn

# Build the contracts using Foundry
RUN source $HOME/.bashrc && forge build