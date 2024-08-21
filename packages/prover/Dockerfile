FROM python:3.10
# FROM whoisgautxm/tachyon:latest AS tachyon
# FROM rust:latest
# FROM node:18


RUN apt-get update && apt-get upgrade -y
# Update the package list and install necessary dependencies
RUN apt-get update && \
  apt install -y  pkg-config gnupg libssl-dev libgmp-dev libsodium-dev python3 python3-pip zip git awscli gcc nodejs npm build-essential

# Node install
RUN npm install -g n
RUN n 18
RUN npm install -g yarn snarkjs

# Set Python 3 as the default python
RUN ln -s /usr/bin/python3 /usr/bin/python

RUN pip install numpy

RUN git clone https://github.com/zkemail/email-wallet.git
WORKDIR /email-wallet/packages/prover
RUN pip install -r requirements.txt
RUN cp ./circom_proofgen.sh /root
WORKDIR /root

# RUN mkdir params
# RUN cp /email-wallet/packages/prover/params/account_creation.wasm /root/params
# RUN cp /email-wallet/packages/prover/params/account_init.wasm /root/params
# RUN cp /email-wallet/packages/prover/params/account_transport.wasm /root/params
# RUN cp /email-wallet/packages/prover/params/claim.wasm /root/params
# RUN cp /email-wallet/packages/prover/params/email_sender.wasm /root/params

RUN mkdir params
WORKDIR /root/params
RUN gdown "https://drive.google.com/uc?id=1T1VvuaZhm1pSQoUkjaZptRbjq1XX8aT8"
RUN unzip params.zip
WORKDIR /root
RUN ls params

# RUN mv build params
# RUN curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-creation/contributions/emailwallet-account-creation_00019.zkey --output ./params/account_creation.zkey
# RUN curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-init/contributions/emailwallet-account-init_00007.zkey --output ./params/account_init.zkey
# RUN curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-transport/contributions/emailwallet-account-transport_00005.zkey --output ./params/account_transport.zkey
# RUN curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-claim/contributions/emailwallet-claim_00006.zkey --output ./params/claim.zkey
# RUN curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-email-sender/contributions/emailwallet-email-sender_00006.zkey --output ./params/email_sender.zkey

RUN chmod +x circom_proofgen.sh
RUN mkdir build



# Install Bazel
RUN curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > bazel.gpg && \
  mv bazel.gpg /etc/apt/trusted.gpg.d/ && \
  echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list && \
  apt-get update && \
  apt-get install -y bazel-6.3.0 && \
  ln -s /usr/bin/bazel-6.3.0 /usr/local/bin/bazel


# Clone and build Tachyon
RUN git clone --branch v0.3.0 https://github.com/kroma-network/tachyon /root/tachyon
WORKDIR /root/tachyon/vendors/circom
RUN CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index && bazel build --@kroma_network_tachyon//:has_openmp -c opt --config linux //:prover_main
