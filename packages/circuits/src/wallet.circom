
pragma circom 2.1.5;

include "circomlib/circuits/bitify.circom";
include "@zk-email/circuits/helpers/sha.circom";
include "@zk-email/circuits/helpers/rsa.circom";
include "@zk-email/circuits/helpers/base64.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "@zk-email/circuits/regexes/from_regex.circom";
include "@zk-email/circuits/regexes/tofrom_domain_regex.circom";
include "@zk-email/circuits/regexes/body_hash_regex.circom";
include "@zk-email/circuits/regexes/subject_regex_operation.circom";
include "@zk-email/circuits/regexes/message_id_regex.circom";
include "@zk-email/circuits/email-verifier.circom";


// Here, n and k are the biginteger parameters for RSA
// This is because the number is chunked into k pack_size of n bits each
template EmailWallet(max_header_bytes, max_body_bytes, n, k, pack_size, calculate_from, expose_to, expose_emails_anon) {
    assert(calculate_from < 2); // 1 if we should expose the from, 0 if we should not
    assert(expose_emails_anon < 2);
    assert(expose_to == 0); // 1 if we should expose the to, 0 if we should not: due to hotmail restrictions, we force-disable this

    signal input in_padded[max_header_bytes]; // prehashed email data, includes up to 512 + 64? bytes of padding pre SHA256, and padded with lots of 0s at end after the length
    signal input modulus[k]; // rsa pubkey, verified with smart contract + DNSSEC proof. split up into k parts of n bits each.
    signal input signature[k]; // rsa signature. split up into k parts of n bits each.
    signal input in_len_padded_bytes; // length of in email data including the padding, which will inform the sha256 block length

    // Header reveal vars
    // TODO: In reality, this max value is 320, and would allow people to break our gaurantees and spoof arbitrary email addresses by registering disgustingly subdomains and going past the end of the 30
    var max_email_len = 31;
    var max_subject_amount_len = max_email_len;
    var max_subject_amount_packed_bytes = count_packed(max_subject_amount_len, pack_size);
    var max_subject_currency_len = 5;
    var max_subject_currency_packed_bytes = count_packed(max_subject_currency_len, pack_size);
    var max_subject_recipient_len = max_email_len;
    var max_subject_recipient_packed_bytes = count_packed(max_subject_recipient_len, pack_size);
    var max_subject_command_len = 10;
    var max_subject_command_packed_bytes = count_packed(max_subject_command_len, pack_size);
    var max_message_id_len = 128;
    var max_email_from_len = max_email_len;
    var max_email_recipient_len = max_email_len;

    signal input command_idx;
    signal input amount_idx;
    signal input currency_idx;
    signal input recipient_idx;
    signal output reveal_command_packed[max_subject_command_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    signal output reveal_amount_packed[max_subject_amount_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
    signal output reveal_currency_packed[max_subject_currency_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space

    // Identity commitment variables
    // Note that you CANNOT use --O1 with this circuit, as it will break the malleability protection: circom 2.1.5: "Improving --O1 simplification: removing signals that do not appear in any constraint and avoiding unnecessary constraint normalizations."
    signal input nullifier;
    signal input relayer;
    signal input body_hash_idx;

    // Verify email signature
    // ignore_body_hash_check is set to true as we dont care about body contents
    component verifier = EmailVerifier(max_header_bytes, max_body_bytes, n, k, 1);
    verifier.in_padded <== in_padded;
    verifier.modulus <== modulus;
    verifier.signature <== signature;
    verifier.in_len_padded_bytes <== in_len_padded_bytes;
    verifier.body_hash_idx <== body_hash_idx;

    // SUBJECT HEADER REGEX: 736,553 constraints
    // This extracts the subject, and the precise regex format can be viewed in the README
    signal subject_regex_out, subject_regex_reveal_command[max_header_bytes], subject_regex_reveal_amount[max_header_bytes], subject_regex_reveal_currency[max_header_bytes], subject_regex_reveal_recipient[max_header_bytes];
    (subject_regex_out, subject_regex_reveal_command, subject_regex_reveal_amount, subject_regex_reveal_currency, subject_regex_reveal_recipient) <== WalletSubjectRegex(max_header_bytes)(in_padded);
    log(subject_regex_out);
    subject_regex_out === 1;

    reveal_command_packed <== ShiftAndPack(max_header_bytes, max_subject_command_len, pack_size)(subject_regex_reveal_command, command_idx);
    reveal_amount_packed <== ShiftAndPack(max_header_bytes, max_subject_amount_len, pack_size)(subject_regex_reveal_amount, amount_idx);
    reveal_currency_packed <== ShiftAndPack(max_header_bytes, max_subject_currency_len, pack_size)(subject_regex_reveal_currency, currency_idx);

    // If the recipient is not being anonymously salted, reveal it
    if(!expose_emails_anon) {
        signal output reveal_recipient_packed[max_subject_recipient_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
        reveal_recipient_packed <== ShiftAndPack(max_header_bytes, max_subject_recipient_len, pack_size)(subject_regex_reveal_recipient, recipient_idx);
    }

    // FROM HEADER REGEX: 736,553 constraints
    // This extracts the from email, and the precise regex format can be viewed in the README
    // TODO: Mitigation for the critical vuln where I can pretend to be another email address by making my email address <max_len_minus_10>@gmail.commydomain.com and <max_len_minus_10>@gmail.com reaches max_len so it truncates is done by ensuring the array index via QuinSelector as such: message_id_regex_reveal[message_id_idx + max_message_id_len] === 0
    if(calculate_from){
        var max_email_from_packed_bytes = count_packed(max_email_from_len, pack_size);
        assert(max_email_from_packed_bytes < max_header_bytes);

        signal input email_from_idx;
        signal email_from[max_email_from_len];

        signal from_regex_out, from_regex_reveal[max_header_bytes];
        (from_regex_out, from_regex_reveal) <== FromRegex(max_header_bytes)(in_padded);
        log(from_regex_out);
        from_regex_out === 1;
        email_from <== VarShiftLeft(max_header_bytes, max_email_from_len)(from_regex_reveal, email_from_idx);

        // If not trying to be anonymous, reveal the from
        if(!expose_emails_anon){
            signal output reveal_email_from_packed[max_email_from_packed_bytes]; // packed into 7-bytes. TODO: make this rotate to take up even less space
            reveal_email_from_packed <== ShiftAndPack(max_header_bytes, max_email_from_len, pack_size)(from_regex_reveal, email_from_idx);
        } else {
            // MESSAGE ID HEADER REGEX: ~736,553 constraints
            // This extracts the email's message ID, and hashes it with the email address
            // TODO: Decompose this into a repeated subfunction to hide intermediate from the top level
            // TODO: Build in spoofing mitigation for https://github.com/zkemail/zk-email-verify/issues/70
            if(expose_emails_anon){
                // Extract Message-ID from header
                signal input message_id_idx;
                signal shifted_message_id[max_message_id_len]; // packed into 7-bytes. TODO: make this rotate to take up even less space

                signal message_id_regex_out, message_id_regex_reveal[max_header_bytes];
                (message_id_regex_out, message_id_regex_reveal) <== MessageIDRegex(max_header_bytes)(in_padded);
                log(message_id_regex_out);
                message_id_regex_out === 1;
                shifted_message_id <== VarShiftLeft(max_header_bytes, max_message_id_len)(message_id_regex_reveal, message_id_idx);
                log(shifted_message_id[0]);

                // FROM ANON ADDRESS
                if(calculate_from){
                    signal input custom_message_id_from[max_message_id_len]; // previous message id, used to source past account
                    signal output (salt_is_message_id_from, custom_anon_from_hashed_salt) <== MakeAnonEmailSalt(max_email_from_len, max_message_id_len)(email_from, custom_message_id_from, shifted_message_id);
                    log(salt_is_message_id_from);
                }

                // RECIPIENT ANON ADDRESS
                // This would be the in-reply-to for the recipient, if it's forwarded to them
                signal wallet_recipient[max_subject_recipient_len] <== VarShiftLeft(max_header_bytes, max_subject_recipient_len)(subject_regex_reveal_recipient, recipient_idx);
                signal input custom_message_id_recipient[max_message_id_len]; // previous message id, used to source past account
                signal output (salt_is_message_id_recipient, custom_anon_recipient_hashed_salt) <== MakeAnonEmailSalt(max_email_recipient_len, max_message_id_len)(wallet_recipient, custom_message_id_recipient, shifted_message_id);
                log(salt_is_message_id_recipient);
            }
        }
    }
}

// Args:
// * max_header_bytes = 1024 is the max number of bytes in the header
// * max_body_bytes = 1536 is the max number of bytes in the body after precomputed slice
// * n = 121 is the number of bits in each chunk of the modulus (RSA parameter)
// * k = 17 is the number of chunks in the modulus (RSA parameter)
// * pack_size = 7 is the number of bytes that can fit into a 255ish bit signal (can increase later)
// * calculate_from = 1 is whether to expose the from email address
// * expose_to = 0 is whether to expose the to email (not recommended)
// * expose_emails_anon = 1 means it will prevent revealing plaintext emails, and instead expose the hash(from/recipient email address, custom message id)
component main { public [ modulus, nullifier, relayer ] } = EmailWallet(1024, 1536, 121, 17, 30, 1, 0, 1);
