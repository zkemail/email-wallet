pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "./utils/constants.circom";
include "./utils/wallet_salt.circom";
// include "./utils/email_addr_pointer.circom";
include "./utils/email_addr_commit.circom";
include "./utils/bytes2ints.circom";

// Verify emailAddr commitmetn and pointer has the same email address.
// Used for claiming unclaimed funds
template Claim() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes];
    signal input cm_rand;
    signal input account_key;

    signal output email_addr_commit;
    signal output wallet_salt;

    // signal recipient_relayer_rand_hash_input[1];
    // recipient_relayer_rand_hash_input[0] <== recipient_relayer_rand;
    // recipient_relayer_rand_hash <== Poseidon(1)(recipient_relayer_rand_hash_input);
    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(email_addr);    
    email_addr_commit <== EmailAddrCommit(num_email_addr_ints)(cm_rand, email_addr_ints);
    wallet_salt <== WalletSalt(num_email_addr_ints)(email_addr_ints, account_key);
}

component main  = Claim();
