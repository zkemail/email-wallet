
pragma circom 2.1.5;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
// include "@zk-email/circuits/email-verifier.circom";
// include "@zk-email/circuits/helpers/extract.circom";
// include "./utils/constants.circom";
// include "./utils/account_salt.circom";
// include "./utils/email_addr_commit.circom";
// include "./utils/hash_sign.circom";
// include "./utils/email_nullifier.circom";
// include "./utils/bytes2ints.circom";
// include "./utils/digit2int.circom";
// include "./regexes/invitation_code_with_prefix_regex.circom";
// include "@zk-email/zk-regex-circom/circuits/common/from_addr_regex.circom";
// include "@zk-email/zk-regex-circom/circuits/common/email_addr_regex.circom";
// include "@zk-email/zk-regex-circom/circuits/common/email_domain_regex.circom";
// include "@zk-email/zk-regex-circom/circuits/common/subject_all_regex.circom";
// include "@zk-email/zk-regex-circom/circuits/common/timestamp_regex.circom";
include "@zk-email/ether-email-auth-circom/src/email_auth_template.circom";
include "@zk-email/ether-email-auth-circom/src/utils/bytes2ints.circom";
include "@zk-email/ether-email-auth-circom/src/utils/constants.circom";

// Verify email from user (sender) and extract subject, timestmap, recipient email (commitment), etc.
// * n - the number of bits in each chunk of the RSA public key (modulust)
// * k - the number of chunks in the RSA public key (n * k > 2048)
// * max_header_bytes - max number of bytes in the email header
// * max_subject_bytes - max number of bytes in the email subject
template EmailSender(n, k, max_header_bytes, max_subject_bytes) {
    signal input padded_header[max_header_bytes]; // email data (only header part)
    signal input public_key[k]; // RSA public key (modulus), k parts of n bits each.
    signal input signature[k]; // RSA signature, k parts of n bits each.
    signal input padded_header_len; // length of in email data including the padding
    signal input sender_account_code;
    signal input from_addr_idx; // Index of the from email address (= sender email address) in the email header
    signal input subject_idx; // Index of the subject in the header
    signal input domain_idx; // Index of the domain name in the from email address
    signal input timestamp_idx; // Index of the timestamp in the header
    signal input code_idx; // index of the invitation code in the header
    signal input recipient_email_idx; // Index of the recipient email address in the subject
    // signal input relayer_rand; // Private randomness of the relayer


    var email_max_bytes = email_max_bytes_const();
    var subject_field_len = compute_ints_size(max_subject_bytes);
    var domain_len = domain_len_const();
    var domain_filed_len = compute_ints_size(domain_len);
    var k2_chunked_size = k >> 1;
    if(k % 2 == 1) {
        k2_chunked_size += 1;
    }
    var timestamp_len = timestamp_len_const();

    signal output domain_name[domain_filed_len];
    signal output public_key_hash;
    signal output email_nullifier;
    signal output timestamp;
    signal output masked_subject[subject_field_len];
    signal output sender_account_salt;
    signal output is_code_exist;
    signal output has_email_recipient;
    signal output recipient_email_addr_commit;
    signal output psi_point[2];

    (domain_name, public_key_hash, email_nullifier, timestamp, masked_subject, sender_account_salt, is_code_exist, has_email_recipient, recipient_email_addr_commit) <== EmailAuth(n, k, max_header_bytes, max_subject_bytes, 1)(padded_header, public_key, signature, padded_header_len, sender_account_code, from_addr_idx, subject_idx, domain_idx, timestamp_idx, code_idx, recipient_email_idx);
}

// Args:
// * n = 121 is the number of bits in each chunk of the modulus (RSA parameter)
// * k = 17 is the number of chunks in the modulus (RSA parameter)
// * max_header_bytes = 1024 is the max number of bytes in the header
// * max_subject_bytes = 605 = 512 + 31*3 is the max number of bytes in the body after precomputed slice. The last 31*3 bytes can be used for the invitation code.
component main  = EmailSender(121, 17, 1024, 605);
