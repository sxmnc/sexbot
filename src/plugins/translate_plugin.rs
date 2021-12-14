use irc::client::prelude::*;
use serde::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct TranslatePlugin {
    triggers: Vec<String>,
    yandex_api_key: String,
}

impl TranslatePlugin {
    pub fn new() -> TranslatePlugin {
        from_config!("config/plugins/translate_config.toml")
    }
}

#[async_trait]
impl super::Plugin for TranslatePlugin {
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
            let mut parts = msg.split_whitespace().skip(1);
            let lang = parts.next();
            let input_text = parts.collect::<Vec<&str>>().join(" ");

            if input_text.len() > 0 {
                match reqwest::Client::new()
                    .get("https://translate.yandex.net/api/v1.5/tr.json/translate")
                    .query(&[
                        ("key", self.yandex_api_key.to_owned()),
                        ("lang", lang.unwrap().to_owned()),
                        ("text", input_text),
                    ])
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<serde_json::Value>().await {
                        Ok(body) => match extract_text(body) {
                            Some(text) => client
                                .send_privmsg(target, format!("Translation result: {}", text))?,
                            None => {}
                        },
                        Err(error) => println!("{}", error),
                    },
                    Err(error) => println!("{}", error),
                }
            }
        }

        Ok(())
    }
}

fn extract_text(value: serde_json::Value) -> Option<String> {
    match value.get("text") {
        Some(text) => match text.as_array() {
            Some(values) => Some(
                values
                    .iter()
                    .map(|s| s.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" "),
            ),
            None => None,
        },
        None => None,
    }
}
