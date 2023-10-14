#!/bin/sh

EMAIL_ADDR=$1
RELAYER_RAND=$2
ACCOUNT_KEY=$3

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/account_creation_input.json"
yarn workspace @email-wallet/circom gen-account-creation-input \
    --email-addr $EMAIL_ADDR \
    --relayer-rand $RELAYER_RAND \
    --account-key $ACCOUNT_KEY \
    --input-file $INPUT_FILE \
    --prove
exit 0