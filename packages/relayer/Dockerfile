# Use the official Rust image as the base image
FROM rust:latest
ARG ZKEMAIL_BRANCH_NAME=anon_wallet
ARG RELAYER_BRANCH_NAME=modal_anon
ARG REFRESH_ZK_EMAIL=0
ARG REFRESH_RELAYER=0

# Install Node.js 16.x and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor -o /usr/share/keyrings/yarn-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/yarn-archive-keyring.gpg] https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list && \
    apt-get update && \
    apt-get install -y yarn

# Update the package list and install necessary dependencies
RUN apt-get update && \
    apt install -y cmake build-essential pkg-config libssl-dev libgmp-dev libsodium-dev nasm

# Clone rapidsnark
RUN  git clone https://github.com/Divide-By-0/rapidsnark /rapidsnark
COPY ./rapidsnark/build /rapidsnark/build
WORKDIR /rapidsnark
RUN npm install
RUN git submodule init
RUN git submodule update
RUN chmod +x /rapidsnark/build/prover
RUN npx task createFieldSources
RUN npx task buildPistache
RUN npx task buildProver

# Clone zk email repository at the latest commit and set it as the working directory
RUN git clone https://github.com/zkemail/zk-email-verify -b ${ZKEMAIL_BRANCH_NAME} /zk-email-verify
COPY ./zk-email-verify/build /zk-email-verify/build
WORKDIR /zk-email-verify
RUN yarn install

# Clone the relayer repository at the latest commit and set it as the working directory
RUN git clone --branch ${RELAYER_BRANCH_NAME} --single-branch https://github.com/zkemail/relayer /relayer
WORKDIR /relayer

# Build for any AWS machine
RUN cargo build --target x86_64-unknown-linux-gnu
RUN cp /relayer/target/x86_64-unknown-linux-gnu/debug/relayer /relayer/target/debug/
RUN cargo build --target x86_64-unknown-linux-gnu --release
RUN cp /relayer/target/x86_64-unknown-linux-gnu/release/relayer /relayer/target/release/

# Update repos to latest commits
RUN git pull
RUN cargo build --target x86_64-unknown-linux-gnu
RUN cp /relayer/target/x86_64-unknown-linux-gnu/debug/relayer /relayer/target/debug/

WORKDIR /zk-email-verify
RUN git pull
RUN yarn install
RUN git pull

# Make necessary files executable
RUN chmod +x /relayer/src/circom_proofgen.sh
