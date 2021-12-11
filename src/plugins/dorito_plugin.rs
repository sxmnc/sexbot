use irc::client::prelude::*;
use rand::Rng;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct DoritoPlugin {
    triggers: Vec<String>,
    message: String,
    false_message: String,
    true_message: String,
    troll_message: String,
}

impl DoritoPlugin {
    pub fn new() -> DoritoPlugin {
        from_config!("config/plugins/dorito_config.toml")
    }
}

impl super::Plugin for DoritoPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.triggers.contains(&msg.trim().to_lowercase())
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            client.send_privmsg(target, &self.message)?;

            let mut rng = rand::thread_rng();

            if rng.gen_range(0..=1) == 0 {
                if rng.gen_range(0..=4) == 0 {
                    client.send_privmsg(target, &self.false_message)?;
                } else {
                    client.send_privmsg(target, &self.true_message)?;
                }
            }

            if rng.gen_range(0..=1) == 0 {
                client.send_privmsg(target, &self.troll_message)?;
            }
        }

        Ok(())
    }
}
