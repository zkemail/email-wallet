
pragma circom 2.1.5;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/email-verifier.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "@zk-email/circuits/regexes/from_regex.circom";
include "circom-grumpkin/circuits/hash_to_curve.circom";
include "./utils/constants.circom";
include "./utils/email_addr_pointer.circom";
include "./utils/viewing_key_commit.circom";
include "./utils/email_addr_wtns.circom";
include "./utils/wallet_salt.circom";
include "./utils/hex2int.circom";
include "./regexes/email_addr_regex.circom";
include "./regexes/email_domain_regex.circom";
include "./regexes/invitation_code_regex.circom";


// Here, n and k are the biginteger parameters for RSA
// This is because the number is chunked into k pack_size of n bits each
template AccountInit(n, k, max_header_bytes) {
    signal input in_padded[max_header_bytes]; // prehashed email data, includes up to 512 + 64? bytes of padding pre SHA256, and padded with lots of 0s at end after the length
    signal input pubkey[k]; // rsa pubkey, verified with smart contract + DNSSEC proof. split up into k parts of n bits each.
    signal input signature[k]; // rsa signature. split up into k parts of n bits each.
    signal input in_padded_len; // length of in email data including the padding, which will inform the sha256 block length
    signal input sender_relayer_rand;
    signal input sender_email_idx; // index of the from email address (= sender email address) in the header
    signal input code_idx; // index of the invitation code in the header
    signal input domain_idx;

    var email_max_bytes = email_max_bytes_const();
    var domain_len = domain_len_const();
    var code_len = invitation_code_len_const();

    signal output domain_name[domain_len];
    signal output pubkey_hash;
    signal output sender_relayer_rand_hash;
    signal output email_nullifier;
    signal output sender_pointer;
    signal output sender_vk_commit;
    signal output psi_point[2];
    // signal output sender_wallet_salt;
    
    var field_pack_bits = field_pack_bits_const();
    var k2_chunked_size = k >> 1;
    if(k % 2 == 1) {
        k2_chunked_size += 1;
    }
    
    component email_verifier = EmailVerifier(max_header_bytes, 0, n, k, 1);
    email_verifier.in_padded <== in_padded;
    email_verifier.modulus <== pubkey;
    email_verifier.signature <== signature;
    email_verifier.in_len_padded_bytes <== in_padded_len;
    signal header_hash[256] <== email_verifier.sha;

    // FROM HEADER REGEX: 736,553 constraints
    signal from_regex_out, from_regex_reveal[max_header_bytes];
    (from_regex_out, from_regex_reveal) <== FromRegex(max_header_bytes)(in_padded);
    log(from_regex_out);
    from_regex_out === 1;
    signal sender_email_addr[email_max_bytes];
    sender_email_addr <== VarShiftLeft(max_header_bytes, email_max_bytes)(from_regex_reveal, sender_email_idx);

    // INVITATION CODE REGEX
    signal code_regex_out, code_regex_reveal[max_header_bytes];
    (code_regex_out, code_regex_reveal) <== InvitationCodeRegex(max_header_bytes)(in_padded);
    log(code_regex_out);
    code_regex_out === 1;
    signal invitation_code_hex[code_len] <== VarShiftLeft(max_header_bytes, code_len)(code_regex_reveal, code_idx);
    signal sender_vk <== Hex2Field()(invitation_code_hex);

    // DOMAIN NAME HEADER REGEX
    signal domain_regex_out, domain_regex_reveal[email_max_bytes];
    (domain_regex_out, domain_regex_reveal) <== EmailDomainRegex(email_max_bytes)(sender_email_addr);
    domain_regex_out === 1;
    domain_name <== VarShiftLeft(email_max_bytes, domain_len)(domain_regex_reveal, domain_idx);

    signal pubkey_hash_input[k2_chunked_size];
    for(var i = 0; i < k2_chunked_size; i++) {
        if(i==k2_chunked_size-1 && k2_chunked_size % 2 == 1) {
            pubkey_hash_input[i] <== pubkey[2*i];
        } else {
            pubkey_hash_input[i] <== pubkey[2*i] + (1<<n) * pubkey[2*i+1];
        }
    }
    pubkey_hash <== Poseidon(k2_chunked_size)(pubkey_hash_input);
    signal sender_relayer_rand_hash_input[1];
    sender_relayer_rand_hash_input[0] <== sender_relayer_rand;
    sender_relayer_rand_hash <== Poseidon(1)(sender_relayer_rand_hash_input);

    signal cm_rand_input[k2_chunked_size];
    for(var i = 0; i < k2_chunked_size; i++) {
        if(i==k2_chunked_size-1 && k2_chunked_size % 2 == 1) {
            cm_rand_input[i] <== signature[2*i];
        } else {
            cm_rand_input[i] <== signature[2*i] + (1<<n) * signature[2*i+1];
        }
    }
    signal cm_rand <== Poseidon(k2_chunked_size)(cm_rand_input);
    signal header_hash_int[field_pack_bits+1];
    header_hash_int[0] <== 0;
    for(var i = 0; i < field_pack_bits; i++) {
        header_hash_int[i+1] <== 2 * header_hash_int[i] + header_hash[i];
    }
    signal email_nullifier_input[2];
    email_nullifier_input[0] <== cm_rand;
    email_nullifier_input[1] <== header_hash_int[field_pack_bits];
    email_nullifier <== Poseidon(2)(email_nullifier_input);

    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal sender_email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(sender_email_addr);
    sender_pointer <== EmailAddrPointer(num_email_addr_ints)(sender_relayer_rand, sender_email_addr_ints);
    sender_vk_commit <== ViewingKeyCommit(num_email_addr_ints)(sender_vk, sender_email_addr_ints, sender_relayer_rand_hash);
    psi_point <== HashToCurve(email_max_bytes)(sender_email_addr);
}

// Args:
// * n = 121 is the number of bits in each chunk of the modulus (RSA parameter)
// * k = 17 is the number of chunks in the modulus (RSA parameter)
// * max_header_bytes = 1024 is the max number of bytes in the header
component main  = AccountInit(121, 17, 1024);
