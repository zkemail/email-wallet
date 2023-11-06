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

RUN git clone https://github.com/zkemail/email-wallet.git -b feat/v1
WORKDIR /email-wallet/packages/prover
RUN pip install -r requirements.txt
RUN cp ./circom_proofgen.sh /root
WORKDIR /root
RUN gdown "https://drive.google.com/uc?id=1qwyksneAArAsuTPC91qTqBDY5nAbP0XH"
RUN unzip params.zip -d "params"
RUN chmod +x circom_proofgen.sh
RUN mkdir build

RUN git clone https://github.com/iden3/rapidsnark.git
WORKDIR /root/rapidsnark
RUN yarn
RUN git submodule init
RUN git submodule update
RUN npx task createFieldSources
RUN npx task buildPistache
RUN npx task buildProver
RUN chmod +x build/prover
