use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct PleurnichePlugin {
    triggers: Vec<String>,
}

impl PleurnichePlugin {
    pub fn new() -> PleurnichePlugin {
        from_config!("config/plugins/pleurniche_config.toml")
    }
}

#[async_trait]
impl super::Plugin for PleurnichePlugin {
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

    async fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            let topic = msg
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");

            if topic.len() > 0 {
                client.send_privmsg(
                    target,
                    format!(
                        "arrêtez de dire que {} !!!!!!! =(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(=(",
                        topic
                    ),
                )?;
            }
        }

        Ok(())
    }
}
