use bellhop::db::Db;
use bellhop::hooks::{Data, Error, ErrorKind, Hook};
use bellhop::models::user::User;

use lettre::{ClientSecurity, SmtpClient, Transport};

use lettre_email::EmailBuilder;

#[derive(Debug)]
pub struct Email;

impl Hook for Email {
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
                )))
            }
        };

        let email = EmailBuilder::new()
            .to((user.email(), "Bellhop User"))
            // ... or by an address only
            .from("sheriff.bellhop@example.com")
            .subject("Bellhop Reservation Expiry Warning")
            .text(format!("This is the bellhop Sheriff letting you know that your reservation (id: {}) is going to expire soon! Best of luck.",user.id()))
            .build()
            .unwrap();

        // Open a local connection on port 25
        let mut mailer = SmtpClient::new("smtp.example.com:25", ClientSecurity::None)
            .unwrap()
            .transport();
        // Send the email
        let result = mailer.send(email.into());

        match result {
            Ok(_) => {}
            Err(e) => println!("Error sending eviction notice: {}", e),
        };

        Ok(())
    }
}
