# Relayer

The relayer sets up the email receiver (IMTP client) and email sender (SMTP client). It then continuosly checks for new emails using the IMTP client. 
When new emails arrive, the are processed. From, body and subject is extracted, and stored as a row in the DB, and also pushed into a queue as Unvalidated emails. 

Next all emails in the queue which have an unvalidated status, are handled. Handling involves writing the email to a sepcific directory.

The relayer is also running the coordinator.py which is watching that directory. If new files are found in that directory, it uploads them to s3 using boto3. 
Then, either generates the proof locally (on the relayer server) or sends the email to another server.

The dockerfile is used to set up the proof gen server while the circom_proofgen.sh is invoked for proof generation. Proof generation involves generating inputs, witness, proof and then forwarding the proof to the chain. 

State is updated to pending for emails for which tx has been sent. Now, we watch for the transaction to mine on chain and once it does, we validate that it's the right nullifer, and then update the status to Ready. The relayer then responds to the sender with a reply email using the email sender client.

The relayer server can be a very small, cheap server with a lot of storage for the database. The proof generation server is spun up on demand as a 64-ish core box and takes only a few seconds to start.