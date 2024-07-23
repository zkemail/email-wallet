pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/ether-email-auth-circom/src/utils/constants.circom";
include "@zk-email/ether-email-auth-circom/src/utils/account_salt.circom";
include "@zk-email/ether-email-auth-circom/src/utils/email_addr_commit.circom";
include "@zk-email/ether-email-auth-circom/src/utils/bytes2ints.circom";

// Verify emailAddr commitmetn and pointer has the same email address.
// Used for claiming unclaimed funds
template Claim() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes];
    signal input cm_rand;
    signal input account_code;

    signal output email_addr_commit;
    signal output account_salt;

    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(email_addr);    
    email_addr_commit <== EmailAddrCommit(num_email_addr_ints)(cm_rand, email_addr_ints);
    account_salt <== AccountSalt(num_email_addr_ints)(email_addr_ints, account_code);
}

component main  = Claim();
