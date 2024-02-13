pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "./utils/constants.circom";
include "./utils/email_addr_commit.circom";
include "./utils/bytes2ints.circom";

// Prove the commitment to an email address using a randomness
// Can be used to register unclaimed funds with announcement
template Announcement() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes];
    signal input cm_rand;

    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal output email_addr_ints[num_email_addr_ints];
    signal output email_addr_commit;

    email_addr_ints <== Bytes2Ints(email_max_bytes)(email_addr);
    email_addr_commit <== EmailAddrCommit(num_email_addr_ints)(cm_rand, email_addr_ints);
}

component main { public [cm_rand] } = Announcement();
