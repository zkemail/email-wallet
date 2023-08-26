
pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";

template HashPubkeyAndSign(n,k) {
    signal input pubkey[k];
    signal input signature[k];

    signal output pubkey_hash;
    signal output cm_rand;

    var k2_chunked_size = k >> 1;
    if(k % 2 == 1) {
        k2_chunked_size += 1;
    }

    signal pubkey_hash_input[k2_chunked_size];
    for(var i = 0; i < k2_chunked_size; i++) {
        if(i==k2_chunked_size-1 && k2_chunked_size % 2 == 1) {
            pubkey_hash_input[i] <== pubkey[2*i];
        } else {
            pubkey_hash_input[i] <== pubkey[2*i] + (1<<n) * pubkey[2*i+1];
        }
    }
    pubkey_hash <== Poseidon(k2_chunked_size)(pubkey_hash_input);
    signal cm_rand_input[k2_chunked_size];
    for(var i = 0; i < k2_chunked_size; i++) {
        if(i==k2_chunked_size-1 && k2_chunked_size % 2 == 1) {
            cm_rand_input[i] <== signature[2*i];
        } else {
            cm_rand_input[i] <== signature[2*i] + (1<<n) * signature[2*i+1];
        }
    }
    cm_rand <== Poseidon(k2_chunked_size)(cm_rand_input);
}



