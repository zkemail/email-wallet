
pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";

// accountKeyCommit = hash(accountKey, emailAddr||0..0, hash(relayerRand))
template AccountKeyCommit(num_ints) {
    signal input account_key;
    signal input email_addr_ints[num_ints];
    signal input relayer_rand_hash;
    signal output ak_commit;
    
    component poseidon = Poseidon(2+num_ints);
    poseidon.inputs[0] <== account_key;
    for(var i=0; i<num_ints; i++) {
        poseidon.inputs[1+i] <== email_addr_ints[i];
    }
    poseidon.inputs[1+num_ints] <== relayer_rand_hash;
    ak_commit <== poseidon.out;
}

