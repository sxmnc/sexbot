use std::collections::HashMap;

use irc::client::prelude::*;
use rand::Rng;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct QuotesPlugin {
    triggers: Vec<String>,
    quotes: HashMap<String, String>,
}

impl QuotesPlugin {
    pub fn new() -> QuotesPlugin {
        from_config!("config/plugins/quotes_config.toml")
    }
}

impl super::Plugin for QuotesPlugin {
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
            let topic = msg
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");

            if topic.len() > 0 {
                if self.quotes.contains_key(&topic) {
                    client.send_privmsg(target, &self.quotes[&topic])?;
                } else {
                    client.send_privmsg(target, format!("No quote tagged \"{}\"", topic))?;
                }
            } else {
                let mut rng = rand::thread_rng();
                let keys = self.quotes.keys().collect::<Vec<&String>>();
                let topic = keys[rng.gen_range(0..keys.len())];
                client.send_privmsg(target, &self.quotes[topic])?;
            }
        }

        Ok(())
    }
}
