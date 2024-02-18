FROM rustlang/rust:nightly

RUN apt-get update && apt-get upgrade -y 
RUN apt-get update && \
    apt install -y cmake build-essential pkg-config libssl-dev libgmp-dev libsodium-dev nasm git awscli gcc nodejs npm vim 

# Node install
RUN npm install -g n 
RUN n 18
RUN npm install -g yarn

RUN git clone https://github.com/zkemail/email-wallet.git
WORKDIR /email-wallet
RUN yarn

WORKDIR /email-wallet/packages/relayer
COPY packages/relayer/.env ./.env
# COPY packages/relayer/scripts/ ./scripts # FIXME: It's for testing
RUN cargo build --release

# WORKDIR /email-wallet/packages/prover
# RUN apt-get update && apt-get install -y python3.10 python3-distutils python3-pip python3-apt
# RUN pip install modal
# ARG modal_token_id
# ARG modal_token_secret
# RUN modal token set --token-id ${modal_token_id} --token-secret ${modal_token_secret} 
# RUN nohup modal serve modal_server.py &

WORKDIR /email-wallet/packages/relayer
CMD [ "/bin/bash", "-c", "/email-wallet/packages/relayer/scripts/startup.sh"]


# # ------------------ Chef stage -------------------
# # Use cargo chef to cache dependencies
# FROM rustlang/rust:nightly AS chef

# # Install cargo chef
# # RUN cargo install cargo-chef 

# # Work in app
# WORKDIR /app

# # ------------------ Planner stage -------------------
# FROM chef as planner

# # Copy files into container
# COPY Cargo.toml Cargo.lock ./
# COPY packages/relayer ./packages/relayer
# COPY packages/utils ./packages/utils
# COPY packages/circuits ./packages/circuits
# COPY packages/frontend ./packages/frontend
# COPY packages/scripts ./packages/scripts

# # Create a lockfile for cargo chef
# RUN cargo +nightly chef prepare --recipe-path recipe.json

# # ------------------ Builder stage -------------------
# FROM chef AS builder
# WORKDIR /relayer

# # Copy over our lock file
# COPY --from=planner  /app /relayer

# # Build for any AWS machine. Same as cargo build but caches dependencies with the chef to make builds faster.
# RUN cargo chef cook --release --recipe-path recipe.json

# ### Above this all dependencies should be cached as long as our lock file stays the same

# # Build binary
# RUN cargo build --release

# # ------------------ Runtime stage -------------------

# # Using super lightweight debian image to reduce overhead
# FROM ubuntu:latest AS runtime

# # Copy prebuild bin from the Builder stage
# COPY --from=builder /relayer/target/release/relayer /relayer/target/release/relayer
# COPY --from=builder /relayer/Cargo.toml /relayer/Cargo.toml
# COPY --from=builder /relayer/Cargo.lock /relayer/Cargo.lock

# EXPOSE 4500

# CMD ["relayer/target/release/relayer"]

# # This cargo chef logic comes from https://github.com/LukeMathWalker/cargo-chef
# # Inspired by Huff: https://github.com/huff-language/huff-rs/blob/main/Dockerfile
