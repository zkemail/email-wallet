#!/bin/bash
set -e # Stop on error

mkdir -p build
mkdir -p params

npm install -g snarkjs@latest
pip install -r requirements.txt
gdown "https://drive.google.com/uc?id=1b49VLOoUsf5c2bFXSQwx1eGIX7_g30TN"
unzip params.zip -d "params"
chmod +x circom_proofgen.sh
