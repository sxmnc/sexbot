use std::collections::HashMap;

use irc::client::prelude::*;

use crate::prelude::*;

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

impl HelpPlugin {
    pub fn new(config: &Config) -> HelpPlugin {
        HelpPlugin {
            trigger: get_required!(config, "help_trigger").to_lowercase(),
            topics: collect_topics(&config.options),
        }
    }
}

impl Plugin for HelpPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.split_whitespace()
            .next()
            .map(|handle| handle.to_lowercase() == self.trigger)
            .unwrap_or_default()
    }

    fn call(
        &self,
        client: &Client,
        target: &str,
        msg: &str,
        _prefix: String,
    ) -> irc::error::Result<()> {
        let sender = client.sender();
        let mut parts = msg.split_whitespace();
        parts.next();

        let topic = parts.next().unwrap_or("help").to_lowercase();

        if self.topics.contains_key(&topic) {
            sender.send_privmsg(target, &self.topics[&topic])?;
        } else {
            sender.send_privmsg(target, format!("No help available for: {}", topic))?;
        }
        Ok(())
    }
}
