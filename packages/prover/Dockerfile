FROM python:3.10
# FROM aayushg0/rapidsnark:latest AS rapidsnark
# FROM rust:latest
# FROM node:18


RUN apt-get update && apt-get upgrade -y 
# Update the package list and install necessary dependencies
RUN apt-get update && \
    apt install -y cmake build-essential pkg-config libssl-dev libgmp-dev libsodium-dev nasm git awscli gcc nodejs npm 

# Node install
RUN npm install -g n 
RUN n 18
RUN npm install -g yarn snarkjs

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

RUN git clone https://github.com/iden3/rapidsnark-old.git rapidsnark
WORKDIR /root/rapidsnark
RUN yarn
RUN git submodule init
RUN git submodule update
RUN npx task createFieldSources
RUN npx task buildPistache
RUN npx task buildProver
RUN chmod +x build/prover
WORKDIR /root