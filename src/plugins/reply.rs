use irc::client::prelude::*;

use crate::prelude::*;

pub struct ReplyPlugin {
    nickname: String,
}

impl ReplyPlugin {
    pub fn new(config: &Config) -> ReplyPlugin {
        ReplyPlugin {
            nickname: config.nickname().unwrap().to_lowercase(),
        }
    }
}

impl Plugin for ReplyPlugin {
    fn matches(&self, msg: &str) -> bool {
        let msg = msg.to_lowercase();
        msg == self.nickname
            || msg == format!("{}:", self.nickname)
            || msg == format!("{},", self.nickname)
    }

    fn call(
        &self,
        client: &Client,
        target: &str,
        msg: &str,
        prefix: String,
    ) -> irc::error::Result<()> {
        let sender = client.sender();
        let suffix = match msg.chars().last() {
            Some(':') => ":",
            Some(',') => ",",
            _ => "",
        };
        sender.send_privmsg(target, format!("{}{}", prefix, suffix))?;
        Ok(())
    }
}
