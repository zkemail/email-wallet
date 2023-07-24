if [ $# -ne 1 ]; then
    echo "Usage: $0 <nonce>"
    exit 1
fi

# Read nonce from args
NONCE=$1

CIRCUIT_NAME="wallet"

# Set file paths needed
SCRIPT_DIR=$(cd -- "$( dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
PROJECT_ROOT=$(dirname $(dirname "$SCRIPT_DIR"))
CIRCUITS_PATH="${PROJECT_ROOT}/packages/circuits"
CIRCUITS_BUILD_PATH="${PROJECT_ROOT}/packages/circuits/build"
RELAYER_PATH="${PROJECT_ROOT}/packages/relayer"
PROOF_DIR_PATH="${RELAYER_PATH}/proofs"

EML_PATH="${RELAYER_PATH}/received_eml/wallet_${NONCE}.eml"
INPUT_JSON_PATH="${PROOF_DIR_PATH}/input_${NONCE}.json"
WITNESS_PATH="${PROOF_DIR_PATH}/witness_${NONCE}.wtns"

RAPIDSNARK_PROVER_PATH="${PROJECT_ROOT}/lib/rapidsnark/build/prover"

echo "\n"

# Make sure inputs dir exists
mkdir -p "${PROOF_DIR_PATH}" 
echo "✓ Inputs dir: ${PROOF_DIR_PATH}"

# Generate input
npx tsx "${CIRCUITS_PATH}/scripts/generate-input.ts" --email-file="${EML_PATH}" --nonce="${NONCE}" --output="${INPUT_JSON_PATH}" --silent | tee /dev/stderr
status_inputgen=$?
if [ $status_inputgen -ne 0 ]; then
    echo "✘ Input generation failed: ${status_inputgen}"
    exit 1
fi
echo "✓ Input generated successfuly"


# Generate witness
node "${CIRCUITS_BUILD_PATH}/${CIRCUIT_NAME}_js/generate_witness.js" "${CIRCUITS_BUILD_PATH}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${INPUT_JSON_PATH}" "${WITNESS_PATH}" | tee /dev/stderr
status_jswitgen=$?
if [ $status_jswitgen -ne 0 ]; then
    echo "✘ Witness generation failed: ${status_jswitgen}"
    exit 1
fi
echo "✓ Witness generated successfuly"


# Generate proof using Rapidsnark
docker run \
    --rm \
    -it \
    -v ${RELAYER_PATH}/proofs:/tmp \
    -v ${CIRCUITS_BUILD_PATH}:/circuit \
    rapidsnark \
        /circuit/${CIRCUIT_NAME}.zkey \
        /tmp/witness_${NONCE}.wtns \
        /tmp/proof_${NONCE}.json \
        /tmp/public_${NONCE}.json

# TODO: Call relayer with proof
