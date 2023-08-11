pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "./utils/constants.circom";
include "./utils/email_addr_pointer.circom";
include "./utils/viewing_key_commit.circom";
include "./utils/email_addr_wtns.circom";
include "./utils/wallet_salt.circom";

template EmailRecipient() {
    var email_max_bytes = email_max_bytes_const();
    signal input recipient_email_addr[email_max_bytes];
    signal input recipient_relayer_rand;
    signal input recipient_vk;
    signal input cm_rand;
    signal output recipient_relayer_rand_hash;
    signal output recipient_pointer;
    signal output recipient_vk_commit;
    signal output recipient_wallet_salt;
    signal output recipient_email_addr_wtns;
    // signal output recipient_vk_wtns;

    signal recipient_relayer_rand_hash_input[1];
    recipient_relayer_rand_hash_input[0] <== recipient_relayer_rand;
    recipient_relayer_rand_hash <== Poseidon(1)(recipient_relayer_rand_hash_input);
    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal recipient_email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(recipient_email_addr);
    recipient_pointer <== EmailAddrPointer(num_email_addr_ints)(recipient_relayer_rand, recipient_email_addr_ints);
    recipient_vk_commit <== ViewingKeyCommit(num_email_addr_ints)(recipient_vk, recipient_email_addr_ints, recipient_relayer_rand_hash);
    recipient_wallet_salt <== WalletSalt()(recipient_vk, 0);
    recipient_email_addr_wtns <== EmailAddrWtns(num_email_addr_ints)(cm_rand, recipient_email_addr_ints);
    // signal recipient_vk_wtns_input[2];
    // recipient_vk_wtns_input[0] <== cm_rand;
    // recipient_vk_wtns_input[1] <== recipient_vk;
    // recipient_vk_wtns <== Poseidon(2)(recipient_vk_wtns_input);
}

component main  = EmailRecipient();
