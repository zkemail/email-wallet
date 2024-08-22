
FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

# Install Node.js, Yarn and required dependencies
RUN apt-get update \
  && apt-get install -y curl git gnupg build-essential cmake libgmp-dev libsodium-dev nasm \
  && curl --silent --location https://deb.nodesource.com/setup_12.x | bash - \
  && apt-get install -y nodejs

RUN  git clone https://github.com/iden3/rapidsnark.git /rapidsnark
WORKDIR /rapidsnark
RUN npm install
RUN git submodule init
RUN git submodule update
RUN npx task createFieldSources
RUN npx task buildPistache
RUN  apt-get install -y 
RUN npx task buildProver

ENTRYPOINT ["/rapidsnark/build/prover"]
