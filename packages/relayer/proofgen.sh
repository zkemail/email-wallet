#!/bin/bash

set -e # Stop on error

# Ensure nonce is set
if [ $# -ne 1 ]; then
    echo "Usage: $0 <nonce>"
    exit 1
fi

# Read nonce from args
NONCE=$1
CIRCUIT_NAME="wallet"

# Set file paths needed
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
PROJECT_ROOT=$(dirname $(dirname "$SCRIPT_DIR"))
CIRCUITS_PATH="${PROJECT_ROOT}/packages/circuits"
CIRCUITS_BUILD_PATH="${PROJECT_ROOT}/packages/circuits/build"
RELAYER_PATH="${PROJECT_ROOT}/packages/relayer"
PROOF_DIR_PATH="${RELAYER_PATH}/proofs/"
EML_PATH="${RELAYER_PATH}/received_eml/wallet_${NONCE}.eml"
INPUT_JSON_PATH="${PROOF_DIR_PATH}/input_${NONCE}.json"
WITNESS_PATH="${PROOF_DIR_PATH}/witness_${NONCE}.wtns"
ZKEY_PATH="${CIRCUITS_BUILD_PATH}/${CIRCUIT_NAME}.zkey"

# ⏵⏵⏵⏵⏵⏵ NOTE: Comment/Uncomment below lines to use local or docker rapidsnark 
# RAPIDSNARK_PATH="${PROJECT_ROOT}/lib/rapidsnark/build/prover"
RAPIDSNARK_PATH="docker run --rm -i -t -v ${PROOF_DIR_PATH}:${PROOF_DIR_PATH} -v ${CIRCUITS_BUILD_PATH}:/${CIRCUITS_BUILD_PATH} rapidsnark"


# Make sure inputs dir exists
mkdir -p "${PROOF_DIR_PATH}"
echo "✓ Inputs dir: ${PROOF_DIR_PATH}"

# Generate input
npx tsx "${CIRCUITS_PATH}/scripts/generate-input.ts" --email-file="${EML_PATH}" --nonce="${NONCE}" --output="${INPUT_JSON_PATH}" --silent
echo "✓ Input generated successfuly"

# Generate witness
node "${CIRCUITS_BUILD_PATH}/${CIRCUIT_NAME}_js/generate_witness.js" "${CIRCUITS_BUILD_PATH}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${INPUT_JSON_PATH}" "${WITNESS_PATH}"
echo "✓ Witness generated successfuly"

# Generate proof using Rapidsnark - This works for both docker and local rapidsnark
${RAPIDSNARK_PATH} \
        ${ZKEY_PATH} \
        ${PROOF_DIR_PATH}/witness_${NONCE}.wtns \
        ${PROOF_DIR_PATH}/rapidsnark_proof_${NONCE}.json \
        ${PROOF_DIR_PATH}/rapidsnark_public_${NONCE}.json 
echo "✓ Proof generated successfuly"

# Call relayer to send proof to chian
# TODO: Upgrade debug -> release and edit dockerfile to use release
"${PROJECT_ROOT}/target/debug/relayer" chain false "${PROOF_DIR_PATH}" "${NONCE}"
echo "✓ Sent to chain successfuly"

exit 0
