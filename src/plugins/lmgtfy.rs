use std::collections::HashMap;

use irc::client::prelude::*;

#[derive(Default)]
pub struct LmgtfyPlugin {
    triggers: Vec<String>,
}

fn collect_triggers(options: &HashMap<String, String>) -> Vec<String> {
    options
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("lmgtfy_trigger_") {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .collect()
}

impl super::Plugin for LmgtfyPlugin {
    fn configure(&mut self, config: &Config) {
        self.triggers = collect_triggers(&config.options);
    }

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
                .collect::<Vec<&str>>()
                .join(" ");

            if topic.len() > 0 {
                let topic = urlencoding::encode(&topic);
                sender.send_privmsg(target, format!("http://lmgtfy.com/?q={}", topic))?;
            }
        }

        Ok(())
    }
}
