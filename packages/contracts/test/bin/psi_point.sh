#!/bin/sh

EMAIL_ADDR=$1
ACCOUNT_CODE=$2
RELAYER_RAND=$3

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/psi_point_input.json"
yarn workspace @email-wallet/circom gen-psi-point-input \
    --email-addr $EMAIL_ADDR \
    --account-code $ACCOUNT_CODE \
    --relayer-rand $RELAYER_RAND \
    --input-file $INPUT_FILE \
    --prove
exit 0