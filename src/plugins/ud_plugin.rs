use irc::client::prelude::*;
use serde::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct UdPlugin {
    triggers: Vec<String>,
}

impl UdPlugin {
    pub fn new() -> UdPlugin {
        from_config!("config/plugins/ud_config.toml")
    }
}

#[async_trait]
impl super::Plugin for UdPlugin {
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
                match reqwest::Client::new()
                    .get("https://api.urbandictionary.com/v0/define")
                    .query(&[("term", topic)])
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<serde_json::Value>().await {
                        Ok(body) => match extract_definition(body) {
                            Some((def, ex)) => client.send_privmsg(
                                target,
                                format!("Definition: {} | ex: {}", def, ex),
                            )?,
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

fn extract_definition(value: serde_json::Value) -> Option<(String, String)> {
    match value.get("list") {
        Some(list) => match list.as_array() {
            Some(results) => {
                if results.len() > 0 {
                    let def = match results[0].get("definition") {
                        Some(value) => value.as_str().map(|s| s.to_owned()),
                        None => None,
                    };

                    let ex = match results[0].get("example") {
                        Some(value) => value.as_str().map(|s| s.to_owned()),
                        None => None,
                    };

                    match (def, ex) {
                        (Some(def), Some(ex)) => Some((def, ex)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        },
        None => None,
    }
}
