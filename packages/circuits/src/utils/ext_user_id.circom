pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "./constants.circom";
include "./email_addr_pointer.circom";
include "./viewing_key_commit.circom";

template ExtUserId() {
    signal input viewing_key;

    signal output id;

    signal id_input[2];
    id_input[0] <== viewing_key;
    id_input[1] <== 1;
    id <== Poseidon(2)(id_input);
}

