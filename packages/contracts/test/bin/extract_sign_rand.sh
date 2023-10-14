#!/bin/sh

EMAIL_FILE_PATH=$1

SCRIPT_DIR=$(cd $(dirname $0); pwd)
OUTPUT_FILE="${SCRIPT_DIR}/../build_integration/sign_rand.txt"
yarn workspace @email-wallet/circom extract-email-sign-rand \
    --email-file $EMAIL_FILE_PATH \
    --output $OUTPUT_FILE
exit 0