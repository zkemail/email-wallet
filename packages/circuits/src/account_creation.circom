pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/circuits/helpers/extract.circom";
include "./utils/constants.circom";
include "./utils/email_addr_pointer.circom";
include "./utils/viewing_key_commit.circom";
include "./utils/wallet_salt.circom";
include "./utils/ext_account_salt.circom";
include "circom-grumpkin/circuits/hash_to_curve.circom";
include "circom-grumpkin/circuits/point_mul.circom";


template AccountCreation() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes];
    signal input relayer_rand;
    signal input viewing_key;
    signal output relayer_rand_hash;
    signal output pointer;
    signal output vk_commit;
    signal output wallet_salt;
    signal output ext_account_salt;
    signal output psi_point[2];
    signal relayer_rand_hash_input[1];
    relayer_rand_hash_input[0] <== relayer_rand;
    relayer_rand_hash <== Poseidon(1)(relayer_rand_hash_input);
    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(email_addr);
    pointer <== EmailAddrPointer(num_email_addr_ints)(relayer_rand, email_addr_ints);
    vk_commit <== ViewingKeyCommit(num_email_addr_ints)(viewing_key, email_addr_ints, relayer_rand_hash);
    wallet_salt <== WalletSalt()(viewing_key);
    ext_account_salt <== ExtAccountSalt()(viewing_key);
    signal hashed_point[2];
    hashed_point <== HashToCurve(email_max_bytes)(email_addr);
    psi_point <== PointScalarMul(254)(hashed_point, relayer_rand);
}

component main  = AccountCreation();
