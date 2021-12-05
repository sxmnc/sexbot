use std::collections::HashMap;

use irc::client::prelude::*;

use crate::macros::*;

#[derive(Default)]
pub struct HelpPlugin {
    trigger: String,
    topics: HashMap<String, String>,
}

fn collect_topics(options: &HashMap<String, String>) -> HashMap<String, String> {
    options
        .iter()
        .filter_map(|(k, v)| match k.strip_suffix("_help") {
            Some(k) => Some((k.to_owned(), v.to_owned())),
            None => None,
        })
        .collect()
}

impl super::Plugin for HelpPlugin {
    fn configure(&mut self, config: &Config) {
        self.trigger = get_required!(config, "help_trigger").to_lowercase();
        self.topics = collect_topics(&config.options);
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
