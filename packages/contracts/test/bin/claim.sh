#!/bin/sh

EMAIL_ADDR=$1
RELAYER_RAND=$2
EMAIL_ADDR_RAND=$3

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/claim_input.json"
yarn workspace @email-wallet/circom gen-claim-input \
    --email-addr $EMAIL_ADDR \
    --relayer-rand $RELAYER_RAND \
    --email-addr-rand $EMAIL_ADDR_RAND \
    --input-file $INPUT_FILE \
    --prove
exit 0