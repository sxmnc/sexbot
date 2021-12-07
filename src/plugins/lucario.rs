use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct LucarioPlugin {
    triggers: Vec<String>,
    message: String,
}

impl LucarioPlugin {
    pub fn new() -> LucarioPlugin {
        from_config!("priv/config/lucario.toml")
    }
}

impl super::Plugin for LucarioPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.triggers.contains(&msg.trim().to_lowercase())
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let sender = client.sender();
            sender.send_privmsg(target, &self.message)?;
        }

        Ok(())
    }
}
