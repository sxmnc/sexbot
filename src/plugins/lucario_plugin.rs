use irc::client::prelude::*;

static TRIGGER: &str = "$lucario";
static RESPONSE: &str = "The bot cannot do Lucario. Lucario is way too sexy.";

pub struct LucarioPlugin;

impl LucarioPlugin {
    pub fn new() -> LucarioPlugin {
        LucarioPlugin
    }
}

impl super::Plugin for LucarioPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg == TRIGGER
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, RESPONSE)?;
        Ok(())
    }
}
