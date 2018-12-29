//! An implementation of [`bellhop::hooks::Hook`] that sends an email warning
//! when leases are about to expire.
//!
//! ## Routes
//!
//! Provides no routes.
//!
//! ## Catchers
//!
//! Provides no catchers.
//!
//! ## Configuration
//!
//! None yet :(
//!
//! ## Example
//!
//! ```no_run
//! use bellhop::Bellhop;
//! use bellhop_hook_email::Email;
//!
//! fn main() {
//!     Bellhop::default()
//!         .hook(Email::new())
//!         .start()
//! }
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

use bellhop::db::Db;
use bellhop::hooks::{Data, Error, ErrorKind, Hook};
use bellhop::models::user::User;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::client::net::{ClientTlsParameters, DEFAULT_TLS_PROTOCOLS};
use lettre::{ClientSecurity, SmtpClient, Transport};

use lettre_email::EmailBuilder;

use native_tls::TlsConnector;

use rocket::fairing::AdHoc;
use rocket::Rocket;

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ClientSecurityConfig {
    None,
    Opportunistic,
    Required,
    Wrapper,
}

impl Default for ClientSecurityConfig {
    fn default() -> Self {
        ClientSecurityConfig::Required
    }
}

fn default_subject() -> String {
    "Bellhop Reservation Expiry Warning".to_owned()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    from: String,

    #[serde(default = "default_subject")]
    subject: String,

    smtp_host: String,
    smtp_port: u16,

    #[serde(default)]
    smtp_client_security: ClientSecurityConfig,

    #[serde(default)]
    smtp_credentials: Option<Credentials>,

    #[serde(default)]
    smtp_auth_mechanism: Option<Mechanism>,
}

impl Config {
    fn tls_parameters(&self) -> ClientTlsParameters {
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(DEFAULT_TLS_PROTOCOLS[0]));

        let connector = tls_builder.build().unwrap();

        ClientTlsParameters::new(self.smtp_host.clone(), connector)
    }

    pub fn create_client(&self) -> SmtpClient {
        let client_security = match self.smtp_client_security {
            ClientSecurityConfig::None => ClientSecurity::None,
            ClientSecurityConfig::Required => ClientSecurity::Required(self.tls_parameters()),
            ClientSecurityConfig::Wrapper => ClientSecurity::Wrapper(self.tls_parameters()),
            ClientSecurityConfig::Opportunistic => {
                ClientSecurity::Opportunistic(self.tls_parameters())
            }
        };

        let mut client =
            SmtpClient::new((self.smtp_host.as_str(), self.smtp_port), client_security).unwrap();

        if let Some(ref creds) = self.smtp_credentials {
            client = client.credentials(creds.clone());
        }

        if let Some(ref mech) = self.smtp_auth_mechanism {
            client = client.authentication_mechanism(mech.clone());
        }

        client
    }
}

/// Sends email when leases are about to expire.
///
/// See the crate documentation for more information.
#[derive(Debug, Default)]
pub struct Email {
    config: Arc<Mutex<Option<Config>>>,
}

impl Email {
    /// Create a new instance of `Email`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Hook for Email {
    fn prelaunch(&self, rocket: Rocket) -> Rocket {
        let config_slot = self.config.clone();

        rocket.attach(AdHoc::on_attach("Hook Email Config", move |rocket| {
            let config: Config = rocket
                .config()
                .get_extra("hook_email")
                .unwrap()
                .clone()
                .try_into()
                .unwrap();

            *config_slot.lock().unwrap() = Some(config);
            Ok(rocket)
        }))
    }

    fn warned(&self, db: &Db, data: Data) -> Result<(), Error> {
        let lease = data.lease();

        let user = User::by_id(db, lease.user_id())
            .map_err(Error::for_kind(ErrorKind::msg("unable to fetch users")))?;

        let user = match user {
            Some(x) => x,
            None => {
                return Err(Error::with_msg(format!(
                    "No userid found for lease: {:?}",
                    lease
                )));
            }
        };

        let config = self.config.lock().unwrap().as_ref().unwrap().clone();

        let email = EmailBuilder::new()
            .to((user.email(), "Bellhop User"))
            // ... or by an address only
            .from(config.from.as_str())
            .subject(config.subject.as_str())
            .text(format!("This is the bellhop Sheriff letting you know that your reservation (id: {}) is going to expire soon! Best of luck.",user.id()))
            .build()
            .unwrap();

        let mut mailer = config.create_client().transport();

        // Send the email
        let result = mailer.send(email.into());

        match result {
            Ok(_) => {}
            Err(e) => println!("Error sending eviction notice: {}", e),
        };

        Ok(())
    }
}
