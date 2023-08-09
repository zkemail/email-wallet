pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
// include "circom-grumpkin/circuits/hash_to_curve.circom";
include "./utils/constants.circom";
include "./utils/pointer.circom";
include "./utils/indicator.circom";
include "./utils/email_commit.circom";

template AccountCreation() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes];
    signal input relayer_rand;
    signal input viewing_key;
    signal output relayer_rand_hash;
    signal output pointer;
    signal output indicator;

    signal relayer_rand_hash_input[1];
    relayer_rand_hash_input[0] <== relayer_rand;
    relayer_rand_hash <== Poseidon(1)(relayer_rand_hash_input);
    var num_email_ints = compute_ints_size(email_max_bytes);
    signal email_addr_ints[num_email_ints] <== Bytes2Ints(email_max_bytes)(email_addr);
    pointer <== Pointer(num_email_ints)(relayer_rand, email_addr_ints);
    indicator <== Indicator(num_email_ints)(viewing_key, email_addr_ints, relayer_rand_hash);
}

component main  = AccountCreation();
