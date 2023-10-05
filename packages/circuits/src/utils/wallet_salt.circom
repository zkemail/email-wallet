pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";

template WalletSalt() {
    signal input account_key;

    signal output salt;

    signal salt_input[2];
    salt_input[0] <== account_key;
    salt_input[1] <== 0;
    salt <== Poseidon(2)(salt_input);
}

