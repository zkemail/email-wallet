import modal

from core import (
    gen_account_creation_proof,
    # gen_account_init_proof,
    # gen_account_transport_proof,
    gen_claim_proof,
    gen_email_sender_proof,
)


stub = modal.Stub("email-wallet-relayer-v1.1")

image = modal.Image.from_dockerfile("Dockerfile")


@stub.function(
    image=image,
    mounts=[
        modal.Mount.from_local_python_packages("core"),
    ],
    cpu=14,
)
@modal.wsgi_app()
def flask_app():
    from flask import Flask, request, jsonify
    import random
    import sys

    app = Flask(__name__)

    @app.post("/prove/account_creation")
    def prove_account_creation():
        req = request.get_json()
        input = req["input"]
        print(input)
        print(type(input))
        nonce = random.randint(
            0,
            sys.maxsize,
        )
        proof = gen_account_creation_proof(str(nonce), False, input)
        return jsonify(proof)

    # @app.post("/prove/account_init")
    # def prove_account_init():
    #     req = request.get_json()
    #     input = req["input"]
    #     nonce = random.randint(
    #         0,
    #         sys.maxsize,
    #     )
    #     proof = gen_account_init_proof(str(nonce), False, input)
    #     return jsonify(proof)

    # @app.post("/prove/account_transport")
    # def prove_account_transport():
    #     req = request.get_json()
    #     input = req["input"]
    #     nonce = random.randint(
    #         0,
    #         sys.maxsize,
    #     )
    #     proof = gen_account_transport_proof(str(nonce), False, input)
    #     return jsonify(proof)

    @app.post("/prove/claim")
    def prove_claim():
        req = request.get_json()
        input = req["input"]
        nonce = random.randint(
            0,
            sys.maxsize,
        )
        proof = gen_claim_proof(str(nonce), False, input)
        return jsonify(proof)

    @app.post("/prove/email_sender")
    def prove_email_sender():
        req = request.get_json()
        input = req["input"]
        nonce = random.randint(
            0,
            sys.maxsize,
        )
        proof = gen_email_sender_proof(str(nonce), False, input)
        return jsonify(proof)

    return app
