use lettre::{
    message::{
        header::{Cc, From, Header, HeaderName, InReplyTo, ReplyTo, To},
        Mailbox, Mailboxes, MessageBuilder,
    },
    transport::smtp::{
        authentication::Credentials, client::SmtpConnection, commands::*, extension::ClientId,
        SMTP_PORT,
    },
    Address, Message, SmtpTransport, Transport,
};

// use mailparse::Mail;
use crate::config::{SMTP_DOMAIN_NAME_KEY, SMTP_PORT_KEY};
use native_tls::{Protocol, TlsConnector};
use std::error::Error;

#[derive(Clone)]
pub struct EmailSenderClient {
    email_id: String,
    transport: SmtpTransport,
}

impl EmailSenderClient {
    pub fn new(email_id: &str, email_app_password: &str, domain_name: Option<&str>) -> Self {
        let smtp_address = domain_name.unwrap_or("smtp.gmail.com");

        let creds = Credentials::new(email_id.to_owned(), email_app_password.to_owned());

        let mut client = SmtpTransport::relay(smtp_address)
            .unwrap()
            .credentials(creds)
            .build();
        println!("SMTP client initialized");
        Self {
            email_id: email_id.to_owned(),
            transport: client,
        }
    }

    pub fn reply_all(&self, raw_email: &str, reply_body: &str) -> Result<(), Box<dyn Error>> {
        let mut original_to = vec![];
        let mut original_cc = vec![];
        let mut original_from = None;
        let mut in_reply_to = String::new();
        let mut original_subject = String::new();

        for line in raw_email.lines() {
            if line.starts_with("To:") {
                let parsed: Result<To, _> = To::parse(line);
                if let Ok(header) = parsed {
                    original_to.push(header);
                }
            } else if line.starts_with("Cc:") {
                let parsed: Result<Cc, _> = Cc::parse(line);
                if let Ok(header) = parsed {
                    original_cc.push(header);
                }
            } else if line.starts_with("From:") {
                let parsed: Result<From, _> = From::parse(line);
                if let Ok(header) = parsed {
                    original_from = Some(header);
                }
            } else if line.starts_with("Message-ID:") {
                in_reply_to = line.trim_start_matches("Message-ID:").trim().to_string();
            } else if line.starts_with("Subject:") {
                original_subject = line.trim_start_matches("Subject:").trim().to_string();
            }
        }
        println!(
            "Parsed email headers: {:?} {:?} {:?} {:?} {:?}",
            original_to, original_cc, original_from, in_reply_to, original_subject
        );
        // Create the email sender's Mailbox
        let sender = Mailbox::new(
            Some("Relayer".to_string()),
            self.email_id.parse::<Address>()?,
        );

        let mut email = Message::builder()
            .from(sender.clone())
            .subject(format!("Re: {}", original_subject))
            .in_reply_to(in_reply_to);

        let mboxes: Mailboxes = original_from.unwrap().into();
        for mbox in mboxes {
            if mbox.email != sender.email {
                email = email.to(mbox);
            }
        }
        for to in original_to {
            let mboxes: Mailboxes = to.into();
            for mbox in mboxes {
                if (mbox.email == self.email_id.parse::<Address>()?) {
                    continue;
                }
                email = email.to(mbox);
            }
        }

        for cc in original_cc {
            let mboxes: Mailboxes = cc.into();
            for mbox in mboxes {
                if (mbox.email == self.email_id.parse::<Address>()?) {
                    continue;
                }
                email = email.cc(mbox);
            }
        }

        let message = match email.body(reply_body.as_bytes().to_vec()) {
            Ok(m) => m,
            Err(e) => {
                println!("Error building email: {:?}", e);
                return Err(Box::new(e));
            }
        };

        println!("Sending email reply-all: {:?}", message);
        self.transport.send(&message)?;
        println!("Sent email reply!");

        Ok(())
    }
}
