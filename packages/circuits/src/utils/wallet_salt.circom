pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "./constants.circom";
include "./email_addr_pointer.circom";
include "./viewing_key_commit.circom";

template WalletSalt() {
    signal input viewing_key;
    signal input public_tag;
    // signal input cm_rand;
    signal output salt;
    // signal output vk_commit;

    signal salt_input[2];
    salt_input[0] <== viewing_key;
    salt_input[1] <== public_tag;
    salt <== Poseidon(2)(salt_input);
    // signal vk_commit_input[2];
    // vk_commit_input[0] <== cm_rand;
    // vk_commit_input[1] <== viewing_key;
    // vk_commit <== Poseidon(2)(vk_commit_input);
}

