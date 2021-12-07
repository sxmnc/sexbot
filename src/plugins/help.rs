use std::collections::HashMap;

use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct HelpPlugin {
    triggers: Vec<String>,
    topics: HashMap<String, String>,
}

impl HelpPlugin {
    pub fn new() -> HelpPlugin {
        from_config!("priv/config/help.toml")
    }
}

impl super::Plugin for HelpPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.split_whitespace()
                .next()
                .map(|handle| self.triggers.contains(&handle.to_lowercase()))
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            let sender = client.sender();
            let topic = msg
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap_or("help")
                .to_lowercase();

            if self.topics.contains_key(&topic) {
                sender.send_privmsg(target, &self.topics[&topic])?;
            } else {
                sender.send_privmsg(target, format!("No help available for: {}", topic))?;
            }
        }

        Ok(())
    }
}
