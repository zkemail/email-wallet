
pragma circom 2.1.5;

include "circomlib/circuits/poseidon.circom";
include "@zk-email/ether-email-auth-circom/src/utils/constants.circom";
include "@zk-email/ether-email-auth-circom/src/utils/account_salt.circom";
include "@zk-email/ether-email-auth-circom/src/utils/email_addr_commit.circom";
include "@zk-email/ether-email-auth-circom/src/utils/bytes2ints.circom";
include "circom-grumpkin/circuits/hash_to_curve.circom";
include "circom-grumpkin/circuits/point_mul.circom";

// Prove that an account salt and psi points are derived from an email address
template PsiPoints() {
    var email_max_bytes = email_max_bytes_const();
    signal input email_addr[email_max_bytes]; // email address
    signal input account_code; // account code
    signal input relayer_rand; // relayer randomness

    signal output account_salt; // account salt = hash(email_addr, account_code)
    signal output psi_points[2]; // psi_points = hash_to_curve(email_addr) * relayer_rand

    var num_email_addr_ints = compute_ints_size(email_max_bytes);
    signal email_addr_ints[num_email_addr_ints] <== Bytes2Ints(email_max_bytes)(email_addr);    
    account_salt <== AccountSalt(num_email_addr_ints)(email_addr_ints, account_code);

    signal hashed_point[2];
    hashed_point <== HashToCurve(email_max_bytes)(email_addr);
    psi_points <== PointScalarMul(254)(hashed_point, relayer_rand);
}

component main  = PsiPoints();
