use anyhow::{anyhow, Result};
use imap::types::{Fetch, Fetches};
use imap::{Authenticator, Client, Session};
use native_tls::{self, TlsStream};
use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde_json;
use std::io;
use std::net::TcpStream;
use std::slice::Iter;

// We cache the domain name, port, and auth for reconnection on failure
#[derive(Debug)]
pub struct ImapClient {
    imap_session: Session<TlsStream<TcpStream>>,
    domain_name: String,
    port: u16,
    auth: IMAPAuth,
}

#[derive(Debug, Clone)]
pub enum IMAPAuth {
    Password {
        id: String,
        password: String,
    },
    OAuth {
        user_id: String,
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    },
}

pub struct OAuthed {
    user_id: String,
    access_token: String,
}

impl<'a> Authenticator for OAuthed {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user_id, self.access_token
        )
    }
}

impl ImapClient {
    pub async fn construct(domain_name: &str, port: u16, auth: IMAPAuth) -> Result<Self> {
        println!("Trying to construct...");
        let tls = native_tls::TlsConnector::builder().build()?;
        println!("Beginning connection process to IMAP server...");
        let client = imap::ClientBuilder::new(domain_name, port)
            .native_tls()
            .expect("Could not connect to imap server");
        println!("IMAP client connected to {:?} {:?}", domain_name, client);
        let mut imap_session = match auth.clone() {
            IMAPAuth::Password { id, password } => client.login(id, password).map_err(|e| e.0),
            IMAPAuth::OAuth {
                user_id,
                client_id,
                client_secret,
                auth_url,
                token_url,
                redirect_url,
            } => {
                let oauth_client = BasicClient::new(
                    ClientId::new(client_id),
                    Some(ClientSecret::new(client_secret)),
                    AuthUrl::new(auth_url)?,
                    Some(TokenUrl::new(token_url)?),
                )
                .set_redirect_uri(RedirectUrl::new(redirect_url)?);
                let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
                let (auth_url, _) = oauth_client
                    .authorize_url(CsrfToken::new_random)
                    // Set the desired scopes.
                    .add_scope(Scope::new("https://mail.google.com/".to_string()))
                    // Set the PKCE code challenge.
                    .set_pkce_challenge(pkce_challenge)
                    .url();
                println!("Browse to: {}", auth_url);
                let mut auth_code = String::new();
                io::stdin().read_line(&mut auth_code)?;
                let token_result = oauth_client
                    .exchange_code(AuthorizationCode::new(auth_code))
                    // Set the PKCE code verifier.
                    .set_pkce_verifier(pkce_verifier)
                    .request_async(async_http_client)
                    .await?;
                let access_token = serde_json::to_string(token_result.access_token())?;
                let oauthed = OAuthed {
                    user_id,
                    access_token,
                };
                client.authenticate("XOAUTH2", &oauthed).map_err(|e| e.0)
            }
        }?;
        // Turn on debug output so we can see the actual traffic coming
        // from the server and how it is handled in our callback.
        // This wouldn't be turned on in a production build, but is helpful
        // in examples and for debugging.
        imap_session.debug = true;
        imap_session.select("INBOX")?;
        Ok(Self {
            imap_session,
            domain_name: domain_name.to_string(),
            port,
            auth,
        })
    }

    pub async fn wait_new_email(&mut self) -> Result<()> {
        loop {
            if self.idle_wait().await.is_err() {
                println!("Connection reset, reconnecting...");
                self.reconnect().await?;
            } else {
                return Ok(());
            }
        }
    }

    async fn idle_wait(&mut self) -> Result<()> {
        let idle_result = self.imap_session.idle().wait_while(|response| {false});
        match idle_result {
            Ok(reason) => println!("IDLE finished normally {:?}", reason),
            Err(e) => println!("IDLE finished with error {:?}", e),
        };
        Ok(())
    }

    async fn reconnect(&mut self) -> Result<()> {
        let mut retry_count = 0;
        let mut MAX_RETRIES = 5;
        while retry_count < MAX_RETRIES {
            match ImapClient::construct(self.domain_name.as_str(), self.port, self.auth.clone())
                .await
            {
                Ok(new_client) => {
                    self.imap_session = new_client.imap_session;
                    return Ok(());
                }
                Err(e) => {
                    println!("Failed to reconnect: {:?}", e);
                    retry_count += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }
            }
        }
        Err(anyhow!(
            "Failed to reconnect after {} attempts",
            MAX_RETRIES
        ))
    }


    pub async fn retrieve_new_emails(&mut self) -> Result<Vec<Fetches>> {
        loop {
            match self.imap_session.uid_search("UNSEEN") {
                Ok(uids) => {
                    let mut fetches = vec![];
                    for (idx, uid) in uids.into_iter().enumerate() {
                        println!("uid {}", uid);
                        let fetched = self
                            .imap_session
                            .uid_fetch(uid.to_string(), "(BODY[] ENVELOPE)")?;
                        fetches.push(fetched);
                    }
                    return Ok(fetches);
                }
                Err(e) => {
                    println!("Connection reset, reconnecting...");
                    if let Err(e) = self.reconnect().await {
                        return Err(e);
                    }
                }
            }
        }
    }
}


