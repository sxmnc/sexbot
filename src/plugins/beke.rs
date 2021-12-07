use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct BekePlugin {
    nickname: Option<String>,
    triggers: Vec<String>,
    temp_nickname: String,
    message: String,
}

impl BekePlugin {
    pub fn new() -> BekePlugin {
        from_config!("priv/config/beke.toml")
    }
}

impl super::Plugin for BekePlugin {
    fn configure(&mut self, config: &Config) {
        self.nickname = Some(config.nickname().unwrap().to_owned());
    }

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
            sender.send(Command::NICK(self.temp_nickname.to_owned()))?;
            sender.send_privmsg(target, &self.message)?;
            sender.send(Command::NICK(self.nickname.as_ref().unwrap().to_owned()))?;
        }

        Ok(())
    }
}
