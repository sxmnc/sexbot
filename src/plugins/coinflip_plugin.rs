use irc::client::prelude::*;
use rand::Rng;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct CoinflipPlugin {
    triggers: Vec<String>,
    heads_message: String,
    tails_message: String,
}

impl CoinflipPlugin {
    pub fn new() -> CoinflipPlugin {
        from_config!("config/plugins/coinflip_config.toml")
    }
}

impl super::Plugin for CoinflipPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.triggers.contains(&msg.trim().to_lowercase())
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let mut rng = rand::thread_rng();

            if rng.gen_range(0..=1) == 0 {
                client.send_privmsg(target, &self.heads_message)?;
            } else {
                client.send_privmsg(target, &self.tails_message)?;
            }
        }

        Ok(())
    }
}
