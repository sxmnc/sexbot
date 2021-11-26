use irc::client::prelude::*;
use rand::Rng;

static TRIGGER: &str = ":^)";
static RESPONSE: &str = "THE TROLL STRUCK AGAIN";
static FALSE_RESPONSE: &str = "False story";
static TRUE_RESPONSE: &str = "True story";
static TROLL_RESPONSE: &str = "TROLOLO";

#[derive(Default)]
pub struct Dorito;

impl super::Plugin for Dorito {
    fn matches(&self, msg: &str) -> bool {
        msg == TRIGGER
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, RESPONSE)?;

        let mut rng = rand::thread_rng();

        if rng.gen_range(0..=1) == 0 {
            if rng.gen_range(0..=4) == 0 {
                sender.send_privmsg(target, FALSE_RESPONSE)?;
            } else {
                sender.send_privmsg(target, TRUE_RESPONSE)?;
            }
        }

        if rng.gen_range(0..=1) == 0 {
            sender.send_privmsg(target, TROLL_RESPONSE)?;
        }
        Ok(())
    }
}
