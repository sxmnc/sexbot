use irc::client::prelude::*;

static TRIGGER: &str = "$lucario";
static RESPONSE: &str = "The bot cannot do Lucario. Lucario is way too sexy.";

#[derive(Default)]
pub struct Lucario;

impl super::Plugin for Lucario {
    fn matches(&self, msg: &str) -> bool {
        msg == TRIGGER
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, RESPONSE)?;
        Ok(())
    }
}
