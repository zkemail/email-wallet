#!/bin/sh

EMAIL_FILE_PATH=$1
OLD_RELAYER_HASH=$2
NEW_RELAYER_RAND=$3

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/account_transport_input.json"
yarn workspace @email-wallet/circom gen-account-transport-input \
    --email-file $EMAIL_FILE_PATH \
    --old-relayer-hash $OLD_RELAYER_HASH \
    --new-relayer-rand $NEW_RELAYER_RAND \
    --input-file $INPUT_FILE \
    --prove
exit 0