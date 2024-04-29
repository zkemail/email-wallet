#!/bin/bash
set -e # Stop on error

mkdir -p build

npm install -g snarkjs@latest
pip install -r requirements.txt
gdown "https://drive.google.com/uc?id=1T1VvuaZhm1pSQoUkjaZptRbjq1XX8aT8"
unzip params.zip
# curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-creation/contributions/emailwallet-account-creation_00019.zkey --output /root/params/account_creation.zkey
# curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-init/contributions/emailwallet-account-init_00007.zkey --output /root/params/account_init.zkey
# curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-account-transport/contributions/emailwallet-account-transport_00005.zkey --output /root/params/account_transport.zkey
# curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-claim/contributions/emailwallet-claim_00006.zkey --output /root/params/claim.zkey
# curl https://email-wallet-trusted-setup-ceremony-pse-p0tion-production.s3.eu-central-1.amazonaws.com/circuits/emailwallet-email-sender/contributions/emailwallet-email-sender_00006.zkey --output /root/params/email_sender.zkey
chmod +x circom_proofgen.sh
