import modal
import sys
import time
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
import subprocess
import os
from dotenv import load_dotenv
import boto3
import requests
from urllib.parse import urlencode, quote
from enum import Enum

# Prover type

class Prover(Enum):
    LOCAL = "local"
    MODAL_CLOUD = "cloud"
    MODAL_ENDPOINT = "modal_endpoint"
    MODAL_STUB = "modal_stub_deprecated"


load_dotenv()  # Load environment variables from .env file
PROVER_LOCATION = Prover(os.getenv('PROVER_LOCATION', Prover.MODAL_ENDPOINT.value))
bucket_name = "relayer-emails"  # Replace with your S3 bucket name
object_key_template = "emls/wallet_[nonce].txt"  # Replace with the desired object key (name) in S3
s3_url = "https://" + bucket_name + ".s3.amazonaws.com/" + object_key_template

# ----------------- LOCAL ENV -----------------
# Define the paths to the AWS config and credentials files
aws_config_path = os.path.expanduser('~/.aws/config')
aws_credentials_path = os.path.expanduser('~/.aws/credentials')
aws_credentials = {}
env_credentials = {}

# Check if both the config and credentials files exist
if os.path.isfile(aws_config_path) and os.path.isfile(aws_credentials_path):
    print("AWS has been configured.")
    # Read AWS credentials from the environment variables or configuration files
    session = boto3.Session()
    aws_credentials = {
        'AWS_ACCESS_KEY_ID': session.get_credentials().access_key,
        'AWS_SECRET_ACCESS_KEY': session.get_credentials().secret_key
    }
else:
    print("AWS has not been configured. Please run 'aws configure' to set up your AWS credentials.")

# ----------- ENV VARIABLES ------------

env_example_path = "./.env.example"
if os.path.isfile(env_example_path):
    def get_variable_names_from_env_file(file_path=env_example_path):
        variable_names = []
        with open(file_path) as file:
            for line in file:
                # Ignore comments and empty lines
                if line.startswith('#') or line.strip() == '':
                    continue
                # Extract variable name (part before the '=' character)
                variable_name = line.split('=')[0].strip()
                variable_names.append(variable_name)
        return variable_names

    # Load additional environment variables from the .env file
    additional_vars = get_variable_names_from_env_file()
    env_credentials = {}
    load_dotenv()  # Load environment variables from .env file
    for var_name in additional_vars:
        # If it doesnt start with local
        if not var_name.startswith("LOCAL_"):
            var_value = os.getenv(var_name)
            if var_value is not None:
                # TODO: Make this cleaner; remove all uses of a non local/modal path env var
                env_credentials[var_name] = var_value
                var_name = var_name.replace("MODAL_", "")
                env_credentials[var_name] = var_value

# Merge the credentials and aws_credentials dictionaries
merged_credentials = {**env_credentials, **aws_credentials}

# --------- AWS HELPER FUNCTIONS ------------

def send_to_modal(url, nonce):
    # Path 2: Send to modal
    # Construct the URL with query parameters
    query_params = {
        'aws_url': url,  # Convert bytes to string for URL encoding
        'nonce': nonce
    }
    encoded_params = urlencode(query_params, quote_via=quote)
    webhook_url = f"https://ziztuww--aayush-pull-and-prove-email.modal.run?{encoded_params}"

    # Send the POST request
    response = requests.post(
        webhook_url,
        headers={'Content-Type': 'application/octet-stream'},
    )

    # Check the status code of the response
    if response.status_code == 200:
        # Read the response body
        response_body = response.text
        # Handle the successful response (e.g., print the response body)
        print(f"Modal response: {response_body}")
    elif response.status_code == 400:
        # Handle the bad request error (e.g., print an error message)
        print("Bad request to Modal")
    else:
        # Handle other status codes (e.g., print a generic error message)
        print("An error occurred on Modal...")


def download_and_write_file(s3_url, nonce):
    # Extract the bucket name and object key from the S3 URL
    s3_url_parts = s3_url.replace("https://", "").replace("[nonce]", nonce).split("/")
    bucket_name = s3_url_parts[0].split(".")[0]
    object_key = "/".join(s3_url_parts[1:])

    # Create an S3 client using boto3
    s3_client = boto3.client("s3")

    # Download the object from S3
    response = s3_client.get_object(Bucket=bucket_name, Key=object_key)
    file_contents = response['Body'].read().decode('utf-8')
    print("File pulled from AWS: ", file_contents)

    # Write the file_contents to the file named after the nonce
    file_name = f"./relayer/received_eml/wallet_{nonce}.eml"
    with open(file_name, 'w') as file:
        file.write(file_contents)
    print("Email file contents: ", file_contents)


def upload_file_to_s3(local_file_path, bucket_name, nonce):
    object_key = object_key_template.replace("[nonce]", nonce)

    # Create an S3 client using boto3
    s3_client = boto3.client('s3')

    # Upload the file to S3 with private access (default)
    s3_client.upload_file(local_file_path, bucket_name, object_key, ExtraArgs={'ACL': 'private'})

    # Print a success message
    print(f"File '{local_file_path}' successfully uploaded to {bucket_name}/{object_key} as a private object.")

    return s3_url.replace("[nonce]", nonce)


# --------- MODAL CLOUD COORDINATOR ------------
image = modal.Image.from_dockerhub(
    "aayushg0/zkemail-modal:modal",
    setup_dockerfile_commands=["RUN apt-get install -y python3 python-is-python3 python3-pip", "RUN cp -r /rapidsnark /root/rapidsnark",
                               "RUN cp -r /relayer /root/relayer",
                               "RUN cp -r /zk-email-verify /root/zk-email-verify"], force_build=True).pip_install_from_requirements("requirements.txt")
stub = modal.Stub(image=image)


@stub.function(cpu=4, image=image)
def prove_email(file_contents: str, nonce: str):
    # Write the file_contents to the file named after the nonce
    file_name = f"/root/relayer/received_eml/wallet_{nonce}.eml"
    with open(file_name, 'w') as file:
        file.write(file_contents)
    print("file_contents: ", file_contents)

    # Print the output of the 'proofgen' command
    circom_script_path = "/root/relayer/src/circom_proofgen.sh"
    print("proofgen modal python output; ")
    subprocess.run([circom_script_path, nonce], check=False, text=True)
    return len(file_contents)


# Create and deploy the secret containing AWS credentials and additional environment variables
stub['credentials_secret'] = modal.Secret.from_dict(merged_credentials)


@stub.function(cpu=14, memory=6000, secret=stub['credentials_secret'])
@modal.web_endpoint(method="POST")
def pull_and_prove_email(aws_url: str, nonce: str):
    download_and_write_file(aws_url, nonce)
    # Print the output of the 'proofgen' command
    new_env = os.environ.copy()
    subprocess.run(["/relayer/src/circom_proofgen.sh", nonce], text=True, env=new_env)
    return

# --------- LOCAL COORDINATOR ------------


def is_eml_file(file_name):
    _, file_extension = os.path.splitext(file_name)
    return file_extension.lower() == '.eml'


class DirectoryChangeHandler(FileSystemEventHandler):
    def on_created(self, event):
        if not event.is_directory:
            print(f"New file {event.src_path} has been added.")
            file_name = os.path.basename(event.src_path)
            if (is_eml_file(file_name)):
                # with open(event.src_path, 'r') as file:
                #     email_content = file.read()
                nonce = file_name[file_name.find('wallet_') + 7:file_name.rfind('.')]
                aws_url = upload_file_to_s3(event.src_path, bucket_name, nonce)

                if PROVER_LOCATION == Prover.LOCAL:
                    subprocess.run(["./src/circom_proofgen.sh", nonce])
                elif PROVER_LOCATION == Prover.MODAL_ENDPOINT or PROVER_LOCATION == Prover.MODAL_CLOUD:
                    send_to_modal(aws_url, nonce)
                elif PROVER_LOCATION == Prover.MODAL_STUB:
                    # Note that this is deprecated and will be removed in the future
                    with stub.run():
                        prove_email.call(aws_url, nonce)


@stub.local_entrypoint()
def prove_on_email(path: str):
    event_handler = DirectoryChangeHandler()
    observer = Observer()
    observer.schedule(event_handler, path, recursive=False)
    observer.start()

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()

    observer.join()


if __name__ == "__main__":
    load_dotenv()  # Load environment variables from .env file

    path = os.getenv("LOCAL_INCOMING_EML_PATH")
    if path is None:
        print("Error: LOCAL_INCOMING_EML_PATH is not set in the .env file")
        sys.exit(1)
    else:
        print("Monitoring directory: ", path)

    prove_on_email(path)
