use irc::client::prelude::*;

use crate::prelude::*;

pub struct LucarioPlugin {
    trigger: String,
    message: String,
}

impl LucarioPlugin {
    pub fn new(config: &Config) -> LucarioPlugin {
        LucarioPlugin {
            trigger: get_required!(config, "lucario_trigger").to_lowercase(),
            message: get_required!(config, "lucario_message"),
        }
    }
}

impl Plugin for LucarioPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.to_lowercase() == self.trigger
    }

    fn call(
        &self,
        client: &Client,
        target: &str,
        _msg: &str,
        _prefix: String,
    ) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, &self.message)?;
        Ok(())
    }
}
