use irc::client::prelude::*;

pub struct ReplyPlugin {
    nickname: Option<String>,
}

impl ReplyPlugin {
    pub fn new() -> ReplyPlugin {
        ReplyPlugin { nickname: None }
    }
}

impl super::Plugin for ReplyPlugin {
    fn configure(&mut self, config: &Config) {
        self.nickname = Some(config.nickname().unwrap().to_lowercase());
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            match msg
                .trim()
                .to_lowercase()
                .strip_prefix(self.nickname.as_ref().unwrap())
            {
                Some(":" | "," | "") => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            let sender = client.sender();
            let source_nickname = message.source_nickname();

            let msg_owned = msg.trim().to_lowercase();
            let message_tail = msg_owned.strip_prefix(self.nickname.as_ref().unwrap());

            if let (Some(nickname), Some(tail @ (":" | "," | ""))) = (source_nickname, message_tail)
            {
                sender.send_privmsg(target, nickname.to_owned() + tail)?;
            }
        }

        Ok(())
    }
}
