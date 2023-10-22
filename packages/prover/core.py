import subprocess
import os
import json

def gen_account_creation_proof(nonce: str, is_local:bool, input:dict) -> dict:
    circuit_name = "account_creation"
    store_input(circuit_name, nonce, input)
    gen_proof(circuit_name, nonce, is_local)
    load_proof(circuit_name, nonce)

def gen_account_init_proof(nonce: str, is_local:bool, input:dict) -> dict:
    circuit_name = "account_init"
    store_input(circuit_name, nonce, input)
    gen_proof(circuit_name, nonce, is_local)
    load_proof(circuit_name, nonce)

def gen_account_transport_proof(nonce: str, is_local:bool, input:dict) -> dict:
    circuit_name = "account_transport"
    store_input(circuit_name, nonce, input)
    gen_proof(circuit_name, nonce, is_local)
    load_proof(circuit_name, nonce)

def gen_claim_proof(nonce: str, is_local:bool, input:dict) -> dict:
    circuit_name = "claim"
    store_input(circuit_name, nonce, input)
    gen_proof(circuit_name, nonce, is_local)
    load_proof(circuit_name, nonce)

def gen_email_sender_proof(nonce: str, is_local:bool, input:dict) -> dict:
    circuit_name = "email_sender"
    store_input(circuit_name, nonce, input)
    gen_proof(circuit_name, nonce, is_local)
    load_proof(circuit_name, nonce)

def store_input(circuit_name:str, nonce: str, json_data:dict):
    cur_dir = get_cur_dir()
    build_dir = os.path.join(cur_dir, "build")
    json_file_path = os.path.join(build_dir, "input_" + circuit_name + "_" + nonce + ".json")
    with open(json_file_path, "w") as json_file:
        json_file.write(json.dumps(json_data, indent=4))

def load_proof(circuit_name:str, nonce: str) -> dict:
    cur_dir = get_cur_dir()
    build_dir = os.path.join(cur_dir, "build")
    json_file_path = os.path.join(build_dir, "rapidsnark_proof_" + circuit_name + "_" + nonce + ".json")
    with open(json_file_path, "r") as json_file:
        return json.loads(json_file.read())

def gen_proof(circuit_name:str, nonce: str, is_local:bool):
    is_local_int:int = 1 if is_local else 0
    cur_dir = get_cur_dir()
    params_dir = os.path.join(cur_dir, "params")
    build_dir = os.path.join(cur_dir, "build")
    subprocess.run([os.path.join(cur_dir, "circom_proofgen.sh"), circuit_name, nonce, params_dir, build_dir, str(is_local_int)],shell=True)

def get_cur_dir() -> str:
    return os.path.dirname(os.path.abspath(__file__))