#!/bin/bash

CIRCUIT_NAME="wallet"
if source "/home/ubuntu/relayer/.env"; then
    echo "Sourced from /home/ubuntu/relayer/.env"
elif source "/root/relayer/.env"; then
    echo "Sourcing from /home/ubuntu/relayer/.env failed, sourced from /root/relayer/.env"
elif source "./.env"; then
    echo "Sourcing from /home/ubuntu/relayer/.env and /root/relayer/.env failed, sourced from ./.env"
else
    echo "Sourcing from /home/ubuntu/relayer/.env, /root/relayer/.env, and ./.env failed, please write args to /root/relayer/.env"
fi

if [ $# -ne 1 ]; then
    echo "Usage: $0 <nonce>"
    exit 1
fi

nonce=$1
zk_email_path="${MODAL_ZK_EMAIL_CIRCOM_PATH}"
HOME="${MODAL_ZK_EMAIL_CIRCOM_PATH}/../"
wallet_eml_dir_path=$MODAL_INCOMING_EML_PATH

if [ "$PROVER_TYPE" = "local" ]; then
    zk_email_path=$LOCAL_ZK_EMAIL_CIRCOM_PATH
    HOME="${LOCAL_ZK_EMAIL_CIRCOM_PATH}/../"
    wallet_eml_dir_path=$LOCAL_INCOMING_EML_PATH
fi

prover_output_path="${wallet_eml_dir_path}/../proofs/"

wallet_eml_path="${wallet_eml_dir_path}/wallet_${nonce}.eml"
build_dir="${zk_email_path}/build/${CIRCUIT_NAME}"
input_wallet_path="${wallet_eml_dir_path}/input_${nonce}.json"
witness_path="${build_dir}/witness_${nonce}.wtns"
proof_path="${prover_output_path}/rapidsnark_proof_${nonce}.json"
public_path="${prover_output_path}/rapidsnark_public_${nonce}.json"

echo "npx tsx ${zk_email_path}/src/scripts/generate_input.ts --email_file='${wallet_eml_path}' --nonce='${nonce}'"
npx tsx "${zk_email_path}/src/scripts/generate_input.ts" --email_file="${wallet_eml_path}" --nonce="${nonce}" | tee /dev/stderr
status_inputgen=$?

echo "Finished input gen! Status: ${status_inputgen}"
if [ $status_inputgen -ne 0 ]; then
    echo "generate_input.ts failed with status: ${status_inputgen}"
    exit 1
fi

echo "node ${build_dir}/${CIRCUIT_NAME}_js/generate_witness.js ${build_dir}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${input_wallet_path} ${witness_path}"
node "${build_dir}/${CIRCUIT_NAME}_js/generate_witness.js" "${build_dir}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${input_wallet_path}" "${witness_path}"  | tee /dev/stderr

status_jswitgen=$?
echo "status_jswitgen: ${status_jswitgen}"
if [ $status_jswitgen -ne 0 ]; then
    echo "generate_witness.js failed with status: ${status_jswitgen}"
    exit 1
fi

# echo "/${build_dir}/${CIRCUIT_NAME}_cpp/email ${input_wallet_path} ${witness_path}"
# "/${build_dir}/${CIRCUIT_NAME}_cpp/email" "${input_wallet_path}" "${witness_path}"
# status_c_wit=$?

# echo "Finished C witness gen! Status: ${status_c_wit}"
# if [ $status_c_wit -ne 0 ]; then
#     echo "C based witness gen failed with status (might be on machine specs diff than compilation): ${status_c_wit}"
#     exit 1
# fi
echo "ldd ${HOME}/rapidsnark/build/prover"
ldd "${HOME}/rapidsnark/build/prover"
status_lld=$?

if [ $status_lld -ne 0 ]; then
    echo "lld prover dependencies failed with status: ${status_lld}"
    exit 1
fi

echo "${HOME}/rapidsnark/build/prover ${build_dir}/${CIRCUIT_NAME}.zkey ${witness_path} ${proof_path} ${public_path}"
"${HOME}/rapidsnark/build/prover" "${build_dir}/${CIRCUIT_NAME}.zkey" "${witness_path}" "${proof_path}" "${public_path}"  | tee /dev/stderr
status_prover=$?

if [ $status_prover -ne 0 ]; then
    echo "prover failed with status: ${status_prover}"
    exit 1
fi

echo "Finished proofgen! Status: ${status_prover}"

# TODO: Upgrade debug -> release and edit dockerfile to use release
echo "${HOME}/relayer/target/debug/relayer chain false ${prover_output_path} ${nonce}"
"${HOME}/relayer/target/debug/relayer" chain false "${prover_output_path}" "${nonce}"  | tee /dev/stderr    
status_chain=$?
if [ $status_chain -ne 0 ]; then
    echo "Chain send failed with status: ${status_chain}"
    exit 1
fi

echo "Finished send to chain! Status: ${status_chain}"
exit 0
