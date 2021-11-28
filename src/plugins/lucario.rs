use irc::client::prelude::*;

use crate::prelude::*;

pub struct LucarioPlugin {
    trigger: String,
    message: String,
}

impl LucarioPlugin {
    pub fn new(config: &Config) -> LucarioPlugin {
        LucarioPlugin {
            trigger: get_required!(config, "lucario_trigger"),
            message: get_required!(config, "lucario_message"),
        }
    }
}

impl Plugin for LucarioPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg == self.trigger
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, &self.message)?;
        Ok(())
    }
}
