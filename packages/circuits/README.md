# Circuit Architecture
## Circom Circuits
### Main Circuits

We provide five main circuits as follows.

#### `account_creation.circom`
A circuit to verify the account creation.

It takes as input the following data:
1. an email address `email_addr`.
2. a relayer's randomness `relayer_rand`.
3. an account key `account_code`. 

Its instances are as follows:
1. the poseidon hash of the relayer's randomness (randomness hash) `relayer_rand_hash`.
2. an email address pointer `pointer`.
3. a commitment of the account key `ak_commit`.
4. a user's wallet salt `account_salt`, a PSI point on elliptic curve `psi_point`.

#### `account_init.circom`
A circuit to verify the account initialization.

It takes as input the following data:
1. a padded email header `in_padded`.
2. an RSA public key `pubkey`.
3. an RSA signature `signature`.
4. the bytes of the padded email header `in_padded_len`.
5. the sender relayer's randomness `sender_relayer_rand`.
6. a starting position of the sender's email address in the email header `sender_email_idx`.
7. a starting position of the invitation code in the email header `code_idx`.
8. a starting position of the email domain in the sender's email address `domain_idx`.
9. a starting position of the timestamp in the email header `timestamp_idx`.

Its instances are as follows:
1. an email domain `domain_name`.
2. a poseidon hash of the RSA public key `pubkey_hash`.
3. the sender relayer's randomness hash `sender_relayer_rand_hash`.
4. an nullifier of the email `email_nullifier`.
5. the sender's email address pointer `sender_pointer`.
6. a commitment of the sender's account key `sender_ak_commit`.
7. a timestamp in the email header `timestamp`.

#### `account_transport.circom`
A circuit to verify the account transport.

It takes as input the following data:
1. a padded email header `in_padded`.
2. an RSA public key `pubkey`.
3. an RSA signature `signature`.
4. the bytes of the padded email header `in_padded_len`.
5. the old relayer's randomness hash `old_relayer_rand_hash`.
6. the new relayer's randomness `new_relayer_rand`.
7. a starting position of the sender's email address in the email header `sender_email_idx`.
8. a starting position of the invitation code in the email header `code_idx`.
9. a starting position of the email domain in the sender's email address `domain_idx`.
10. a starting position of the timestamp in the email header `timestamp_idx`.

Its instances are as follows:
1. an email domain `domain_name`.
2. a poseidon hash of the RSA public key `pubkey_hash`.
3. an nullifier of the email `email_nullifier`.
4. a commitment of the sender's account key under the old relayer `old_ak_commit`.
5. a commitment of the sender's account key under the new relayer `new_ak_commit`.
6. the new relayer's randomness hash `new_relayer_rand_hash`.
7. a timestamp in the email header `timestamp`.
8. the old relayer's randomness hash `old_relayer_rand_hash`.

#### `claim.circom`
A circuit to verify the claim of unclaimed funds/states. 

It takes as input the following data.
1. an email address `recipient_email_addr`,
2. a relayer's randomness `recipient_relayer_rand`.
3. a randomness used for the email address commitment `cm_rand`.

Its instances are as follows:
1. the relayer's randomness hash `recipient_relayer_rand_hash`.
2. an email address pointer `recipient_pointer`.
3. an email address commitment `recipient_email_addr_commit`.

#### `email_sender.circom`
A circuit to verify the user's email for calling a command. 

It takes as input the following data.
1. a padded email header `in_padded`.
2. an RSA public key `pubkey`.
3. an RSA signature `signature`.
4. the bytes of the padded email header `in_padded_len`.
5. the sender relayer's randomness `sender_relayer_rand`.
6. a starting position of the sender's email address in the email header `sender_email_idx`.
7. a starting position of the subject in the email header `subject_idx`.
8. a starting position of the recipient's email address in the subject `recipient_email_idx`.
9. a starting position of the email domain in the sender's email address `domain_idx`.
10. a starting position of the timestamp in the email header `timestamp_idx`.

Its instances are as follows:
1. a masked subject where a character in the email address is replaced with zero  `masked_subject_str`.
2. an email domain `domain_name`.
3. a poseidon hash of the RSA public key `pubkey_hash`.
4. the sender relayer's randomness hash `sender_relayer_rand_hash`.
5. an nullifier of the email `email_nullifier`.
6. the sender's email address pointer `sender_pointer`.
7. a flag whether the subject contains the recipient's email address `has_email_recipient` (0 or 1).
8. the recipient's email address commitment `recipient_email_addr_commit`.
9. a timestamp in the email header `timestamp`.
    
Constraint breakdown of the circuit is as follows:
1. this file is a standard template designed for email verification `email-verifier.circom : 669650 constraints` .
2. this circuit is designed to validate and extract email addresses from a given message `from_addr_regex.circom : 643296 constraints`.
3. this circuit is designed to verify if a given input message conforms to a specific regex pattern that checks for email subjects `subject_all_regex.circom : 99208 constraints`.
4. this circuit is designed to validate email addresses based on a specified regular expression. It checks if the input string (representing an email address) conforms to the defined pattern `email_addr_regex.circom : 92037  constraints`.
5. this circuit is designed to validate an invitation code that may have a prefix. It uses a regular expression to check if the input message conforms to a specific pattern, which includes optional spaces and the word "code" followed by a hexadecimal string `invitation_code_with_prefix_regex.circom :  43494 constraints`.
6. `email_domain_regex.circom :  38219 constraints`.
7. this circuit is designed to validate a specific format of a DKIM (DomainKeys Identified Mail) signature within a given message`timestamp_regex.circom :  233872 constraints`.


#### `announcement.circom`
A circuit to verify that the given email address commitment is derived from the given email address and randomness. While it is not used in our core contracts, it is provided for third-party contracts to register unclaimed funds/states for the public email address and the randomness. 

It takes as input the following data:
1. an email address `email_addr`.
2. a randomness used for the email address commitment `cm_rand`.
    
Its instances are as follows:
1. a packed integer of the email address `email_addr_ints`.
2. an email address commitment `email_addr_commit`.
3. a randomness used for the email address commitment `cm_rand`.

## How to Use
### Build circuits
`yarn && yarn build`

### Run tests
At `packages/circuits`, make a `build` directory, download the zip file from the following link, and place its unziped files under `build`.
https://drive.google.com/file/d/1F1cw9eTlgJwPcXR4wF8kQrWPMhRsFnIy/view?usp=sharing

Then run the following command.
`yarn test`

### Generate proving keys and verifier contracts for main circuits
`yarn dev-setup`

### Generate proofs for main circuits with example inputs
`yarn gen-random-proofs`

### Verify the proofs
`yarn verify-proofs`