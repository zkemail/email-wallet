#!/usr/bin/env python
# encoding: utf-8

import json
from flask import Flask
import jsonify
import request
import random
import sys
from core import (
    gen_account_creation_proof,
    gen_account_init_proof,
    gen_account_transport_proof,
    gen_claim_proof,
    gen_email_sender_proof,
)

app = Flask(__name__)


@app.route("/prove/account_creation", methods=["POST"])
def prove_account_creation():
    req = request.get_json()
    input = req["input"]
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_account_creation_proof(nonce, input)
    return jsonify(proof)


@app.route("/prove/account_init", methods=["POST"])
def prove_account_init():
    req = request.get_json()
    input = req["input"]
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_account_init_proof(nonce, input)
    return jsonify(proof)


@app.route("/prove/account_transport", methods=["POST"])
def prove_account_transport():
    req = request.get_json()
    input = req["input"]
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_account_transport_proof(nonce, input)
    return jsonify(proof)


@app.route("/prove/claim", methods=["POST"])
def prove_claim():
    req = request.get_json()
    input = req["input"]
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_claim_proof(nonce, input)
    return jsonify(proof)


@app.route("/prove/email_sender", methods=["POST"])
def prove_email_sender():
    req = request.get_json()
    input = req["input"]
    nonce = random.randint(
        0,
        sys.maxsize,
    )
    proof = gen_email_sender_proof(nonce, input)
    return jsonify(proof)


if __name__ == "__main__":
    from waitress import serve

    serve(app, host="0.0.0.0", port=8080)
