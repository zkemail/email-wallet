#!/bin/sh

EMAIL_FILE_PATH=$1
RELAYER_RAND=$2

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/account_init_input.json"
yarn workspace @email-wallet/circom gen-account-init-input \
    --email-file $EMAIL_FILE_PATH \
    --relayer-rand $RELAYER_RAND \
    --input-file $INPUT_FILE \
    --prove
exit 0