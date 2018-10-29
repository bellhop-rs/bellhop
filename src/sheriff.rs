use crate::db;
use crate::errors::*;
use crate::models::user::User;

use diesel;
use diesel::prelude::*;

use chrono::prelude::*;

use lettre::smtp::SmtpTransportBuilder;
use lettre::{ClientSecurity, EmailTransport};
use lettre_email::EmailBuilder;

const SMTP_ENDPOINT: &str = "smtp.example.com:25";

pub fn send_eviction_notices(c: &PgConnection) -> Result<()> {
    let now = Utc::now();
    let all_leases = db::get_all_leases(&c)?;

    for lease in all_leases {
        let time_left = lease.end_time() - lease.start_time();
        let margin = time_left / 20;
        if now > (lease.end_time() - margin) {
            let user = match User::by_id(&c, lease.user_id())? {
                Some(x) => x,
                None => bail!("No userid found for lease: {:?}", lease),
            };

            let email = EmailBuilder::new()
                .to((user.email(), "Bellhop User"))
                // ... or by an address only
                .from("sherrif.bellhop@example.com")
                .subject("Bellhop Reservation Expiery Warning")
                .text(format!("This is the bellhop Sherrif letting you know that your reservation (id: {}) is going to expire in {}! Best of luck.",user.id(),time_left))
                .build()
                .unwrap();

            // Open a local connection on port 25
            let mut mailer = SmtpTransportBuilder::new(SMTP_ENDPOINT, ClientSecurity::None)
                .unwrap()
                .build();
            // Send the email
            let result = mailer.send(&email);

            match result {
                Ok(_) => {}
                Err(e) => println!("Error sending eviction notice: {}", e),
            };
        }
    }
    Ok(())
}

pub fn evict(c: &PgConnection) -> Result<()> {
    use crate::schema::leases::dsl::*;

    let num_deleted_rows = match diesel::delete(leases)
        .filter(end_time.lt(Utc::now()))
        .execute(c)
    {
        Ok(x) => x,
        Err(e) => bail!("Error deleting leases: {}", e),
    };

    println!(
        "The sherrif successfully evicted {:?} occupants.",
        num_deleted_rows
    );

    Ok(())
}
