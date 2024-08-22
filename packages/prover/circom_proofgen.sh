#!/bin/bash
set -e # Stop on error

if [ $# -ne 5 ]; then
    echo "Usage: $0 <circuitName> <nonce> <paramsDir> <buildDir> <isLocal>"
    exit 1
fi

circuitName=$1
nonce=$2
paramsDir=$3
buildDir=$4
isLocal=$5
SCRIPT_DIR=$(cd $(dirname $0); pwd)
# zk_email_path="${MODAL_ZK_EMAIL_CIRCOM_PATH}"
# HOME="${MODAL_ZK_EMAIL_CIRCOM_PATH}/../"
# wallet_eml_dir_path=$MODAL_INCOMING_EML_PATH

# prover_output_path="${wallet_eml_dir_path}/../proofs/"

# wallet_eml_path="${wallet_eml_dir_path}/wallet_${nonce}.eml"
# build_dir="${zk_email_path}/build/${CIRCUIT_NAME}"
input_path="${buildDir}/input_${circuitName}_${nonce}.json"
witness_path="${buildDir}/witness_${circuitName}_${nonce}.wtns"
proof_path="${buildDir}/rapidsnark_proof_${circuitName}_${nonce}.json"
public_path="${buildDir}/rapidsnark_public_${circuitName}_${nonce}.json"

cd "${SCRIPT_DIR}"
echo "entered zk email path: ${SCRIPT_DIR}"

echo "NODE_OPTIONS='--max-old-space-size=644000' snarkjs wc "${paramsDir}/${circuitName}.wasm" "${input_path}" "${witness_path}""
NODE_OPTIONS='--max-old-space-size=644000' snarkjs wc "${paramsDir}/${circuitName}.wasm" "${input_path}" "${witness_path}"  | tee /dev/stderr
status_jswitgen=$?
echo "✓ Finished witness generation with js! ${status_jswitgen}"

# TODO: Get C-based witness gen to work
# echo "/${build_dir}/${CIRCUIT_NAME}_cpp/${CIRCUIT_NAME} ${input_wallet_path} ${witness_path}"
# "/${build_dir}/${CIRCUIT_NAME}_cpp/${CIRCUIT_NAME}" "${input_wallet_path}" "${witness_path}"
# status_c_wit=$?

# echo "Finished C witness gen! Status: ${status_c_wit}"
# if [ $status_c_wit -ne 0 ]; then
#     echo "C based witness gen failed with status (might be on machine specs diff than compilation): ${status_c_wit}"
#     exit 1
# fi

if [ $isLocal = 1 ]; then
    # DEFAULT SNARKJS PROVER (SLOW)
    NODE_OPTIONS='--max-old-space-size=644000' snarkjs groth16 prove "${paramsDir}/${circuitName}.zkey" "${witness_path}" "${proof_path}" "${public_path}"
    status_prover=$?
    echo "✓ Finished slow proofgen! Status: ${status_prover}"
else
    # RAPIDSNARK PROVER (10x FASTER)
    echo "ldd ${SCRIPT_DIR}/rapidsnark/build/prover"
    ldd "${SCRIPT_DIR}/rapidsnark/build/prover"
    status_lld=$?
    echo "✓ lld prover dependencies present! ${status_lld}"

    echo "${SCRIPT_DIR}/rapidsnark/build/prover ${paramsDir}/${circuitName}.zkey ${witness_path} ${proof_path} ${public_path}"
    "${SCRIPT_DIR}/rapidsnark/build/prover" "${paramsDir}/${circuitName}.zkey" "${witness_path}" "${proof_path}" "${public_path}"  | tee /dev/stderr
    status_prover=$?
    echo "✓ Finished rapid proofgen! Status: ${status_prover}"
fi



# TODO: Upgrade debug -> release and edit dockerfile to use release
# echo "${HOME}/relayer/target/release/relayer chain false ${prover_output_path} ${nonce}"
# "${HOME}/relayer/target/release/relayer" chain false "${prover_output_path}" "${nonce}" 2>&1 | tee /dev/stderr    
# status_chain=$?
# echo "✓ Finished send to chain! Status: ${status_chain}"

exit 0