#!/bin/sh

EMAIL_FILE_PATH=$1
ACCOUNT_KEY=$2

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/email_sender_input.json"
yarn workspace @email-wallet/circom gen-email-sender-input \
    --email-file $EMAIL_FILE_PATH \
    --account-key $ACCOUNT_KEY \
    --input-file $INPUT_FILE \
    --prove
exit 0