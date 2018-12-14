//! bellhop-bin is an example of how to attach different authentication and hook
//! modules to the bellhop library.
//!
//! While you can run this without customization, it probably won't be what you
//! want.

#![deny(missing_docs)]

use bellhop::Bellhop;

use bellhop_auth_dummy::Dummy;

use bellhop_auth_header::Header;

use bellhop_hook_email::Email;

use bellhop_hook_jenkins::Jenkins;

fn main() {
    Bellhop::default()
        .auth(Header) // Allow logging in based on the value of 'X-Bellhop-Email' header.
        .auth(Dummy) // Allow logging in with just an email address.
        .hook(Email) // Sends emails when a lease is close to expiring.
        .hook(Jenkins) // Triggers a jenkins build on certain hook events.
        .start() // Start running the server.
}
