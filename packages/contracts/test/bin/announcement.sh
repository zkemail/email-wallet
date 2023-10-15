#!/bin/sh

EMAIL_ADDR=$1
EMAIL_ADDR_RAND=$2

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORKSPACE_DIR="${SCRIPT_DIR}/../../"
INPUT_FILE="${SCRIPT_DIR}/../build_integration/announcement_input.json"
yarn workspace @email-wallet/circom gen-announcement-input \
    --email-addr $EMAIL_ADDR \
    --email-addr-rand $EMAIL_ADDR_RAND \
    --input-file $INPUT_FILE \
    --prove
exit 0