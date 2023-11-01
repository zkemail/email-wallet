pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "./utils/constants.circom";
include "./utils/email_addr_pointer.circom";
include "./utils/email_addr_commit.circom";
include "./utils/bytes2ints.circom";

// Verify emailAddr commitmetn and pointer has the same email address.
// Used for claiming unclaimed funds
template Claim() {
    var email_max_bytes = email_max_bytes_const();
    signal input recipient_email_addr[email_max_bytes];
    signal input recipient_relayer_rand;
    signal input cm_rand;
    signal output recipient_relayer_rand_hash;
    signal output recipient_pointer;
    signal output recipient_email_addr_commit;

    signal recipient_relayer_rand_hash_input[1];
    recipient_relayer_rand_hash_input[0] <== recipient_relayer_rand;
    recipient_relayer_rand_hash <== Poseidon(1)(recipient_relayer_rand_hash_input);
    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal recipient_email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(recipient_email_addr);
    recipient_pointer <== EmailAddrPointer(num_email_addr_ints)(recipient_relayer_rand, recipient_email_addr_ints);
    recipient_email_addr_commit <== EmailAddrCommit(num_email_addr_ints)(cm_rand, recipient_email_addr_ints);
}

component main  = Claim();
