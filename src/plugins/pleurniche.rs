use irc::client::prelude::*;

use crate::macros::*;

#[derive(Default)]
pub struct PleurnichePlugin {
    trigger: String,
}

impl super::Plugin for PleurnichePlugin {
    fn configure(&mut self, config: &Config) {
        self.trigger = get_required!(config, "pleurniche_trigger").to_lowercase();
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.split_whitespace()
                .next()
                .map(|handle| handle.to_lowercase() == self.trigger)
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
                .collect::<Vec<&str>>()
                .join(" ");

            if topic.len() > 0 {
                sender.send_privmsg(
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
