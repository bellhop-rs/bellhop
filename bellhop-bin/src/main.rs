use bellhop::Bellhop;

use bellhop_auth_dummy::Dummy;

use bellhop_hook_email::Email;

use bellhop_hook_jenkins::Jenkins;

fn main() {
    Bellhop::default().auth(Dummy).hook(Email).hook(Jenkins).start()
}
