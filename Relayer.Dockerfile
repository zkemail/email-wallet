# Use Debian Bullseye as base image with Node.js 18
FROM node:18-bullseye

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

