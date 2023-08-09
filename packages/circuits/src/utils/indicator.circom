
pragma circom 2.1.5;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
include "./constants.circom";
include "./bytes2ints.circom";

// indicator = hash("INDICATOR", viewingKey, emailAddr||0..0, hash(relayerRand))
template Indicator(num_ints) {
    signal input viewing_key;
    signal input email_addr_ints[num_ints];
    signal input relayer_rand_hash;
    signal output indicator;
    
    component poseidon = Poseidon(3+num_ints);
    poseidon.inputs[0] <== indicator_tag();
    poseidon.inputs[1] <== viewing_key;
    for(var i=0; i<num_ints; i++) {
        poseidon.inputs[2+i] <== email_addr_ints[i];
    }
    poseidon.inputs[2+num_ints] <== relayer_rand_hash;
    indicator <== poseidon.out;
}

