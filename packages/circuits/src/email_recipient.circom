pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "./utils/constants.circom";
include "./utils/pointer.circom";
include "./utils/indicator.circom";
include "./utils/email_commit.circom";

template EmailRecipient() {
    var email_max_bytes = email_max_bytes_const();
    signal input recipient_email[email_max_bytes];
    signal input recipient_relayer_rand;
    signal input recipient_viewing_key;
    signal input cm_rand;
    signal output recipient_relayer_rand_hash;
    signal output recipient_pointer;
    signal output recipient_indicator;
    signal output recipient_email_commit;
    signal output recipient_vk_commit;

    signal recipient_relayer_rand_hash_input[1];
    recipient_relayer_rand_hash_input[0] <== recipient_relayer_rand;
    recipient_relayer_rand_hash <== Poseidon(1)(recipient_relayer_rand_hash_input);
     var num_email_ints = compute_ints_size(email_max_bytes);
    signal recipient_email_addr_ints[num_email_ints] <== Bytes2Ints(email_max_bytes)(recipient_email);
    recipient_pointer <== Pointer(num_email_ints)(recipient_relayer_rand, recipient_email_addr_ints);
    recipient_indicator <== Indicator(num_email_ints)(recipient_viewing_key, recipient_email_addr_ints, recipient_relayer_rand_hash);
    recipient_email_commit <== EmailCommit(num_email_ints)(cm_rand, recipient_email_addr_ints);
    signal recipient_vk_commit_input[2];
    recipient_vk_commit_input[0] <== cm_rand;
    recipient_vk_commit_input[1] <== recipient_viewing_key;
    recipient_vk_commit <== Poseidon(2)(recipient_vk_commit_input);
}

component main  = EmailRecipient();
