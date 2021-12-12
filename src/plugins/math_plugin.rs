use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct MathPlugin {
    triggers: Vec<String>,
}

impl MathPlugin {
    pub fn new() -> MathPlugin {
        from_config!("config/plugins/math_config.toml")
    }
}

#[async_trait]
impl super::Plugin for MathPlugin {
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
            let expression = msg
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");

            if expression.len() > 0 {
                match meval::eval_str(expression) {
                    Ok(result) => client.send_privmsg(target, format!("result: {}", result))?,
                    Err(error) => client.send_privmsg(target, error)?,
                }
            }
        }

        Ok(())
    }
}
