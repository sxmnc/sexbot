use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct NohomoPlugin {
    triggers: Vec<String>,
    message: String,
}

impl NohomoPlugin {
    pub fn new() -> NohomoPlugin {
        from_config!("config/plugins/nohomo_config.toml")
    }
}

#[async_trait]
impl super::Plugin for NohomoPlugin {
    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.triggers.iter().any(|trigger| msg.contains(trigger))
        } else {
            false
        }
    }

    async fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            client.send_privmsg(target, &self.message)?;
        }

        Ok(())
    }
}
