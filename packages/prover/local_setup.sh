#!/bin/bash
set -e # Stop on error

mkdir -p build

npm install -g snarkjs@latest
pip install -r requirements.txt
gdown "https://drive.google.com/uc?id=1pe07fwKbtjMLHoQSY8Bcr4rg9aN06mFP"
unzip params.zip
chmod +x circom_proofgen.sh
