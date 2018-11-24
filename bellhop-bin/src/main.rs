use bellhop::Bellhop;

use bellhop_hook_email::Email;

use bellhop_hook_jenkins::Jenkins;

fn main() {
    Bellhop::default().hook(Email).hook(Jenkins).start()
}
