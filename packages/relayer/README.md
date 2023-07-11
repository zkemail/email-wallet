# Relayer

A permissionless Rust IMAP + SMTP server that reads email via imap, creates zk proofs of it, and replies to it. Uses IMAP to receive email and SMTP to send replies.

Goerli Wallet Address (circom-only): 0x3b3857eaf44804cce00449b7fd40310e6de6496e

## Setup

In a new cloud instance, install Rust:

```sh
curl https://sh.rustup.rs -sSf | sh
```

And then:

```sh
sudo apt update
sudo apt install -y pkg-config libssl-dev build-essential libfontconfig1-dev
cargo build --release
chmod +x src/circom_proofgen.sh
```

If you want to run a local prover, also run:

```
chmod +x ../rapidsnark/build/prover
ip -4 -o addr show scope global | awk '{print $4}' && ip -6 -o addr show scope global | awk '{print $4}' # Point the DNS to these raw IPs
```

And make sure that the abi in ` abi/wallet.abi` is up to date.

## Enable TLS/TCP Keepalive

From [here](https://aws.amazon.com/blogs/networking-and-content-delivery/implementing-long-running-tcp-connections-within-vpc-networking/), or else your IMAP connection will drop every 6ish idle minutes. Edit: Apparenly this is not enough.

```sh
echo -e "net.ipv4.tcp_keepalive_time = 45\nnet.ipv4.tcp_keepalive_intvl = 45\nnet.ipv4.tcp_keepalive_probes = 9" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

## Tests

### Test Chain

Test chain connection to verify that your connection to the chain works and simple tx's will send.

```sh
cargo test
```

### Test proofgen

To test local proofgen, send `relayer@sendeth.org` an email while relayer is running then run

```sh
./src/circom_proofgen.sh
```


## Run

First, run the relayer.

```sh
cargo run relayer
```

### Run infra

Then run the prover + infrastructure coordinator.

```sh
sudo apt install python3-pip awscli
pip3 install -r requirements.txt
aws configure
python3 coordinator.py
```

## Update the docker file

Note that this dir structure is only needed if you want to regenerate the docker image.
```
-
  - zk-email-verify
    - ...
  - relayer
    - ...
  - rapidsnark
    - ...
```

Note that you'll have to populate the build folder, run `make` in `zk-email-verify/build/email/email_cpp`, and install rapidsnark according to the zk-email-verify README.

If you want to edit the Dockerfile or compile a new image, copy it one folder up (for now) and setup with these commands:

Setup:

```sh
cp Dockerfile ..
cd ..
sudo docker login
# This command is needed so that git cargo dependencies in docker can be pulled
echo -e '\n[url "ssh://"]\n\tinsteadOf = git://' >> ~/.gitconfig
```

You have to compile the zk-email-verify, relayer, and rapidsnark directories.

Recompile:

```sh
sudo docker build -t zkemail-modal . && sudo docker tag zkemail-modal aayushg0/zkemail-modal:modal && sudo docker push aayushg0/zkemail-modal:modal
```

And edit the tag in coordinator.py. To re-deploy the modal instance, do

```
modal token set --token-id <tokenid> --token-secret <tokensecret>
modal deploy --name aayush coordinator.py
```

To hit it, take the resulting URL and hit

```
curl 'https://ziztuww--aayush-test.modal.run?<query params>'
```

## Server Setup

We don't use this server anymore, but if you'd like to call these functions via endpoints, you can use this nginx setup.

```
sudo apt-get install nginx certbot python3-certbot-nginx
```

### Turn on nginx

````
Configure Nginx: Create a new Nginx configuration file for your application:

```sh
sudo nano /etc/nginx/sites-available/sendeth
````

Paste the following configuration and adjust the domain name and paths accordingly:

```
server {
        listen 80;
        server_name sendeth.org www.sendeth.org;
        return 301 https://$host$request_uri;
}

server {
        listen 443 ssl;
        server_name sendeth.org www.sendeth.org;

        ssl_certificate /etc/letsencrypt/live/sendeth.org/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/sendeth.org/privkey.pem;
    ssl_protocols TLSv1.3 TLSv1.2;
    ssl_prefer_server_ciphers on;
    ssl_dhparam /etc/nginx/dhparam.pem;
        ssl_ciphers 'TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GS256-GCM-SHA384:EECDH+AESGCM:EDH+AESGCM'

        location / {
                proxy_pass http://localhost:3000;
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header X-Forwarded-Proto $scheme;
        }
}
```

We rely on gmail for IMAP, but if you want your own server, you can add thiss:

```
mail {
    server_name sendeth.com;

    imap_capabilities "IMAP4rev1" "UIDPLUS";

    server {
        listen 143;
        protocol imap;
    }

    server {
        listen 993 ssl;
        protocol imap;
        ssl_certificate /etc/letsencrypt/live/sendeth.com/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/sendeth.com/privkey.pem;
    }
}
```

Save and exit the file. Create a symbolic link to enable the site:

```bash
sudo ln -s /etc/nginx/sites-available/sendeth /etc/nginx/sites-enabled/
```

Test the Nginx configuration and restart Nginx:

```sh
export YOURDOMAIN=sendeth.org
sudo certbot --nginx -d $YOURDOMAIN -d www.$YOURDOMAIN
```

### Enable IMAP in Gmail

Here's how to enable IMAP access and use App Passwords for your Gmail or Google Workspace account:

Enable IMAP:

a. Sign in to your Gmail or Google Workspace account.
b. Click the gear icon in the top-right corner and select "See all settings."
c. Go to the "Forwarding and POP/IMAP" tab.
d. In the "IMAP access" section, select "Enable IMAP."
e. Click "Save Changes."

Create an App Password (recommended):

a. Go to your Google Account settings: https://myaccount.google.com/
b. In the left-hand menu, click "Security."
c. In the "Signing in to Google" section, click on "App Passwords." (Note: This option will only be available if you have 2-Step Verification enabled.)
d. Click on "Select app" and choose "Mail" from the dropdown menu.
e. Click on "Select device" and choose the device you're using or select "Other" to enter a custom name.
f. Click "Generate."
g. Google will generate a 16-character App Password. Make sure to copy and save it securely, as you won't be able to see it again.

Now, when connecting to Gmail or Google Workspace via IMAP, use your email address as the "imap id" (username) and the generated App Password as the "password." If you have not enabled 2-Step Verification and are using "Less secure apps" access, you can use your regular email password instead of the App Password. However, using App Passwords is recommended for enhanced security.

### Enable ports in AWS

If there's an error, make sure your ports are open and traffic is allowed. This will be a massive pain in the \*\*\* so just stay with me while 3 hours of your life dissapate to nonsensical setups. Ensure that your EC2 instance has port 80 and 443 open in the security group to allow incoming traffic. You can check and update your security group settings from the AWS EC2 Management Console.

Step 0 is make sure to always add your IPv4 and IPv6 addresses to A and AAAA records respectively for both @ and www in DNS settings of the domain.

Then, enable inbound traffic. To do so, follow these steps:

0. Log in to the AWS Management Console.
1. Navigate to the EC2 Dashboard.
2. Select "Security Groups" from the left sidebar.
3. Find the security group associated with your EC2 instance and click on its name.
4. Click on the "Inbound rules" tab.
5. For the server, check if there are rules allowing traffic on ports 80 and 443. If not, add the rules by clicking on "Edit inbound rules" and then "Add rule". Choose "HTTP" for port 80 and "HTTPS" for port 443, and set the source to "Anywhere" or "0.0.0.0/0" (IPv4) and "::/0" (IPv6). For IMAP, click on "Add rule" and create new rules for the necessary IMAP ports (143 and 993) with the following settings:

- Type: Custom TCP
- Protocol: TCP
- Port Range: 143 (for IMAP) or 993 (for IMAPS)
- Source: Choose "Anywhere" for both IPv4 and IPv6 (0.0.0.0/0 for IPv4 and ::/0 for IPv6)

0. To rnable IPv6 support for your VPC (Virtual Private Cloud), go to the VPC Dashboard in the AWS Management Console, select your VPC, click on "Actions", and then click on "Edit CIDRs". Add an IPv6 CIDR block.
1. Enable IPv6 support for your subnet. Go to the "Subnets" section in the VPC Dashboard, select the subnet associated with your EC2 instance, click on "Actions", and then click on "Edit IPv6 CIDRs". Add an IPv6 CIDR block.
2. Enable IPv6 support for your EC2 instance. Go to the EC2 Dashboard in the AWS Management Console, select your instance, click on "Actions", and then click on "Manage IP Addresses". Assign an IPv6 address to your instance's network interface.
3. Update your instance's security group to allow inbound IPv6 traffic, if necessary.
4. If needed, configure your operating system to use IPv6. This step depends on the OS you're using. For most Linux distributions, IPv6 is enabled by default.

To enable the security group traffic, run these:

0. Log in to the AWS Management Console and navigate to the EC2 Dashboard: https://console.aws.amazon.com/ec2/
1. In the left-hand menu, click on "Security Groups" under the "Network & Security" section.
2. Locate the security group associated with your instance. You can find the security group in the instance details in the "Instances" section of the EC2 Dashboard.
3. Select the security group and click on the "Inbound rules" tab in the lower panel.
4. Click "Edit inbound rules."
5. Click "Add rule" to create a new rule.
6. Choose the desired rule type (e.g., "HTTP" for port 80, "HTTPS" for port 443, or "Custom TCP" for other ports). In the "Source" field, select "Anywhere-IPv6" to allow traffic from all IPv6 addresses. You can also specify a custom IPv6 range in CIDR notation (e.g., 2001:db8::/32).
7. Click "Save rules" to apply the changes.

Then in AWS EC2 shell, run

```sh
sudo ufw enable
sudo ufw allow http
sudo ufw allow https
sudo ufw allow ssh
sudo ufw allow 3000
```

Then run the certbot command again.
