
pragma circom 2.1.5;

include "circomlib/circuits/bitify.circom";
include "circomlib/circuits/comparators.circom";
include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/sha.circom";
include "@zk-email/circuits/helpers/rsa.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "@zk-email/circuits/regexes/from_regex.circom";
// include "@zk-email/circuits/regexes/tofrom_domain_regex.circom";
// include "@zk-email/circuits/regexes/subject_regex_operation.circom";
include "./utils/constants.circom";
include "./utils/pointer.circom";
include "./utils/indicator.circom";
include "./utils/email_commit.circom";
include "./regexes/email_addr_regex.circom";
include "./regexes/eth_addr_regex.circom";
include "./regexes/subject_all_regex.circom";
include "./regexes/email_domain_regex.circom";

// Here, n and k are the biginteger parameters for RSA
// This is because the number is chunked into k pack_size of n bits each
template EmailSender(n, k, max_header_bytes, max_subject_bytes) {
    assert(max_header_bytes % 64 == 0);
    // assert(max_body_bytes % 64 == 0);
    assert(n * k > 2048); // constraints for 2048 bit RSA
    assert(n < (255 \ 2)); // we want a multiplication to fit into a circom signal
    // assert(calculate_from < 2); // 1 if we should expose the from, 0 if we should not
    // assert(expose_emails_anon < 2);
    // assert(expose_to == 0); // 1 if we should expose the to, 0 if we should not: due to hotmail restrictions, we force-disable this

    signal input in_padded[max_header_bytes]; // prehashed email data, includes up to 512 + 64? bytes of padding pre SHA256, and padded with lots of 0s at end after the length
    signal input pubkey[k]; // rsa pubkey, verified with smart contract + DNSSEC proof. split up into k parts of n bits each.
    signal input signature[k]; // rsa signature. split up into k parts of n bits each.
    signal input in_padded_len; // length of in email data including the padding, which will inform the sha256 block length
    signal input sender_relayer_rand;
    signal input sender_viewing_key;
    signal input sender_email_idx; // index of the from email address (= sender email address) in the header
    signal input subject_idx; // index of the subject in the header
    signal input recipient_email_idx; // index of the recipient email address in the subject
    signal input domain_idx;

    var email_max_bytes = email_max_bytes_const();
    var domain_len = domain_len_const();

    signal output has_email_recipient;
    signal output masked_subject_str[max_subject_bytes];
    signal output has_eth_recipient;
    signal output domain_name[domain_len];
    signal output pubkey_hash;
    signal output sender_relayer_rand_hash;
    signal output email_nullifier;
    signal output sender_pointer;
    signal output sender_indicator;
    signal output sender_vk_commit;
    signal output recipient_email_commit;
    
    var field_pack_bits = field_pack_bits_const();
    var k2_chunked_size = k >> 1;
    if(k % 2 == 1) {
        k2_chunked_size += 1;
    }
    

    // SHA HEADER: 506,670 constraints
    // This calculates the SHA256 hash of the header, which is the "base_msg" that is RSA signed.
    // The header signs the fields in the "h=Date:From:To:Subject:MIME-Version:Content-Type:Message-ID;"
    // section of the "DKIM-Signature:"" line, along with the body hash.
    // Note that nothing above the "DKIM-Signature:" line is signed.
    signal header_hash[256] <== Sha256Bytes(max_header_bytes)(in_padded, in_padded_len);
    var msg_len = (256 + n) \ n;

    component base_msg[msg_len];
    for (var i = 0; i < msg_len; i++) {
        base_msg[i] = Bits2Num(n);
    }
    for (var i = 0; i < 256; i++) {
        base_msg[i \ n].in[i % n] <== header_hash[255 - i];
    }
    for (var i = 256; i < n * msg_len; i++) {
        base_msg[i \ n].in[i % n] <== 0;
    }
    
    // VERIFY RSA SIGNATURE: 149,251 constraints
    // The fields that this signature actually signs are defined as the body and the values in the header
    component rsa = RSAVerify65537(n, k);
    for (var i = 0; i < msg_len; i++) {
        rsa.base_message[i] <== base_msg[i].out;
    }
    for (var i = msg_len; i < k; i++) {
        rsa.base_message[i] <== 0;
    }
    rsa.modulus <== pubkey;
    rsa.signature <== signature;

    // FROM HEADER REGEX: 736,553 constraints
    signal from_regex_out, from_regex_reveal[max_header_bytes];
    (from_regex_out, from_regex_reveal) <== FromRegex(max_header_bytes)(in_padded);
    log(from_regex_out);
    from_regex_out === 1;
    signal sender_email[email_max_bytes];
    sender_email <== VarShiftLeft(max_header_bytes, email_max_bytes)(from_regex_reveal, sender_email_idx);

    // SUBJECT HEADER REGEX
    signal subject_regex_out, subject_regex_reveal[max_header_bytes];
    (subject_regex_out, subject_regex_reveal) <== SubjectAllRegex(max_header_bytes)(in_padded);
    log(subject_regex_out);
    subject_regex_out === 1;
    signal subject_all[max_subject_bytes];
    subject_all <== VarShiftLeft(max_header_bytes, max_subject_bytes)(subject_regex_reveal, subject_idx);
    signal recipient_email_regex_out, recipient_email_regex_reveal[max_subject_bytes];
    (recipient_email_regex_out, recipient_email_regex_reveal) <== EmailAddrRegex(max_subject_bytes)(subject_all);
    log(recipient_email_regex_out);
    has_email_recipient <== IsZero()(recipient_email_regex_out-1);
    signal recipient_email[email_max_bytes];
    recipient_email <== VarShiftLeft(max_subject_bytes, email_max_bytes)(recipient_email_regex_reveal, recipient_email_idx);
    for(var i = 0; i < max_subject_bytes; i++) {
        masked_subject_str[i] <== subject_all[i] - has_email_recipient * recipient_email_regex_reveal[i];
    }
    signal recipient_eth_regex_out, recipient_eth_regex_reveal[max_subject_bytes];
    (recipient_eth_regex_out, recipient_eth_regex_reveal) <== EthAddrRegex(max_subject_bytes)(subject_all);
    has_eth_recipient <== recipient_eth_regex_out;
    has_email_recipient * has_eth_recipient === 0;

    // DOMAIN NAME HEADER REGEX
    signal domain_regex_out, domain_regex_reveal[email_max_bytes];
    (domain_regex_out, domain_regex_reveal) <== EmailDomainRegex(email_max_bytes)(sender_email);
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

    var num_email_ints = compute_ints_size(email_max_bytes);
    signal sender_email_addr_ints[num_email_ints] <== Bytes2Ints(email_max_bytes)(sender_email);
    sender_pointer <== Pointer(num_email_ints)(sender_relayer_rand, sender_email_addr_ints);
    sender_indicator <== Indicator(num_email_ints)(sender_viewing_key, sender_email_addr_ints, sender_relayer_rand_hash);
    signal sender_vk_commit_input[2];
    sender_vk_commit_input[0] <== cm_rand;
    sender_vk_commit_input[1] <== sender_viewing_key;
    sender_vk_commit <== Poseidon(2)(sender_vk_commit_input);
    signal cm_rand2_input[1];
    cm_rand2_input[0] <== cm_rand;
    signal cm_rand2 <== Poseidon(1)(cm_rand2_input);
    signal recipient_email_addr_ints[num_email_ints] <== Bytes2Ints(email_max_bytes)(recipient_email);
    recipient_email_commit <== EmailCommit(num_email_ints)(cm_rand2, recipient_email_addr_ints);


    






    // // Header reveal vars
    // // TODO: In reality, this max value is 320, and would allow people to break our gaurantees and spoof arbitrary email addresses by registering disgustingly subdomains and going past the end of the 30
    // var max_email_len = 31;
    // var max_subject_amount_len = max_email_len;
    // var max_subject_amount_packed_bytes = count_packed(max_subject_amount_len, pack_size);
    // var max_subject_currency_len = 5;
    // var max_subject_currency_packed_bytes = count_packed(max_subject_currency_len, pack_size);
    // var max_subject_recipient_len = max_email_len;
    // var max_subject_recipient_packed_bytes = count_packed(max_subject_recipient_len, pack_size);
    // var max_subject_command_len = 10;
    // var max_subject_command_packed_bytes = count_packed(max_subject_command_len, pack_size);
    // var max_message_id_len = 128;
    // var max_email_from_len = max_email_len;
    // var max_email_recipient_len = max_email_len;

    // // signal input command_idx;
    // // signal input amount_idx;
    // // signal input currency_idx;
    // // signal input recipient_idx;
    // signal output reveal_command_packed[max_subject_command_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    // signal output reveal_amount_packed[max_subject_amount_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    // signal output reveal_currency_packed[max_subject_currency_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    

    // // Identity commitment variables
    // // Note that you CANNOT use --O1 with this circuit, as it will break the malleability protection: circom 2.1.5: "Improving --O1 simplification: removing signals that do not appear in any constraint and avoiding unnecessary constraint normalizations."
    // // signal input nullifier;
    // // signal input relayer;
    // // signal input body_hash_idx;

    // // Verify email signature
    // // ignore_body_hash_check is set to true as we dont care about body contents
    // component verifier = EmailVerifier(max_header_bytes, max_body_bytes, n, k, 1);
    // verifier.in_padded <== in_padded;
    // verifier.modulus <== modulus;
    // verifier.signature <== signature;
    // verifier.in_len_padded_bytes <== in_len_padded_bytes;
    // verifier.body_hash_idx <== body_hash_idx;

    // // SUBJECT HEADER REGEX: 736,553 constraints
    // // This extracts the subject, and the precise regex format can be viewed in the README
    // signal subject_regex_out, subject_regex_reveal_command[max_header_bytes], subject_regex_reveal_amount[max_header_bytes], subject_regex_reveal_currency[max_header_bytes], subject_regex_reveal_recipient[max_header_bytes];
    // (subject_regex_out, subject_regex_reveal_command, subject_regex_reveal_amount, subject_regex_reveal_currency, subject_regex_reveal_recipient) <== WalletSubjectRegex(max_header_bytes)(in_padded);
    // log(subject_regex_out);
    // subject_regex_out === 1;

    // reveal_command_packed <== ShiftAndPack(max_header_bytes, max_subject_command_len, pack_size)(subject_regex_reveal_command, command_idx);
    // reveal_amount_packed <== ShiftAndPack(max_header_bytes, max_subject_amount_len, pack_size)(subject_regex_reveal_amount, amount_idx);
    // reveal_currency_packed <== ShiftAndPack(max_header_bytes, max_subject_currency_len, pack_size)(subject_regex_reveal_currency, currency_idx);

    // // If the recipient is not being anonymously salted, reveal it
    // if(!expose_emails_anon) {
    //     signal output reveal_recipient_packed[max_subject_recipient_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    //     reveal_recipient_packed <== ShiftAndPack(max_header_bytes, max_subject_recipient_len, pack_size)(subject_regex_reveal_recipient, recipient_idx);
    // }

    // // FROM HEADER REGEX: 736,553 constraints
    // // This extracts the from email, and the precise regex format can be viewed in the README
    // // TODO: Mitigation for the critical vuln where I can pretend to be another email address by making my email address <max_len_minus_10>@gmail.commydomain.com and <max_len_minus_10>@gmail.com reaches max_len so it truncates is done by ensuring the array index via QuinSelector as such: message_id_regex_reveal[message_id_idx + max_message_id_len] === 0
    // if(calculate_from){
    //     var max_email_from_packed_bytes = count_packed(max_email_from_len, pack_size);
    //     assert(max_email_from_packed_bytes < max_header_bytes);

    //     signal input email_from_idx;
    //     signal email_from[max_email_from_len];

    //     signal from_regex_out, from_regex_reveal[max_header_bytes];
    //     (from_regex_out, from_regex_reveal) <== FromRegex(max_header_bytes)(in_padded);
    //     log(from_regex_out);
    //     from_regex_out === 1;
    //     email_from <== VarShiftLeft(max_header_bytes, max_email_from_len)(from_regex_reveal, email_from_idx);

    //     // If not trying to be anonymous, reveal the from
    //     if(!expose_emails_anon){
    //         signal output reveal_email_from_packed[max_email_from_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    //         reveal_email_from_packed <== ShiftAndPack(max_header_bytes, max_email_from_len, pack_size)(from_regex_reveal, email_from_idx);
    //     } else {
    //         // MESSAGE ID HEADER REGEX: ~736,553 constraints
    //         // This extracts the email's message ID, and hashes it with the email address
    //         // TODO: Decompose this into a repeated subfunction to hide intermediate from the top level
    //         // TODO: Build in spoofing mitigation for https://github.com/zkemail/zk-email-verify/issues/70
    //         if(expose_emails_anon){
    //             // Extract Message-ID from header
    //             signal input message_id_idx;
    //             signal shifted_message_id[max_message_id_len]; // packed into 7-bytes. TODO: make this rotate to take up even less space

    //             signal message_id_regex_out, message_id_regex_reveal[max_header_bytes];
    //             (message_id_regex_out, message_id_regex_reveal) <== MessageIDRegex(max_header_bytes)(in_padded);
    //             log(message_id_regex_out);
    //             message_id_regex_out === 1;
    //             shifted_message_id <== VarShiftLeft(max_header_bytes, max_message_id_len)(message_id_regex_reveal, message_id_idx);
    //             log(shifted_message_id[0]);

    //             // FROM ANON ADDRESS
    //             if(calculate_from){
    //                 signal input custom_message_id_from[max_message_id_len]; // previous message id, used to source past account
    //                 signal output (salt_is_message_id_from, custom_anon_from_hashed_salt) <== MakeAnonEmailSalt(max_email_from_len, max_message_id_len)(email_from, custom_message_id_from, shifted_message_id);
    //                 log(salt_is_message_id_from);
    //             }

    //             // RECIPIENT ANON ADDRESS
    //             // This would be the in-reply-to for the recipient, if it's forwarded to them
    //             signal wallet_recipient[max_subject_recipient_len] <== VarShiftLeft(max_header_bytes, max_subject_recipient_len)(subject_regex_reveal_recipient, recipient_idx);
    //             signal input custom_message_id_recipient[max_message_id_len]; // previous message id, used to source past account
    //             signal output (salt_is_message_id_recipient, custom_anon_recipient_hashed_salt) <== MakeAnonEmailSalt(max_email_recipient_len, max_message_id_len)(wallet_recipient, custom_message_id_recipient, shifted_message_id);
    //             log(salt_is_message_id_recipient);
    //         }
    //     }
    // }
}

// Args:
// * n = 121 is the number of bits in each chunk of the modulus (RSA parameter)
// * k = 17 is the number of chunks in the modulus (RSA parameter)
// * max_header_bytes = 1024 is the max number of bytes in the header
// * max_subject_bytes = 512 is the max number of bytes in the body after precomputed slice
component main  = EmailSender(121, 17, 1024, 512);
