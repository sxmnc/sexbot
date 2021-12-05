use irc::client::prelude::*;

#[derive(Default)]
pub struct ReplyPlugin {
    nickname: String,
}

impl super::Plugin for ReplyPlugin {
    fn configure(&mut self, config: &Config) {
        self.nickname = config.nickname().unwrap().to_lowercase();
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            match msg.trim().to_lowercase().strip_prefix(&self.nickname) {
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
            let message_tail = msg_owned.strip_prefix(&self.nickname);

            if let (Some(nickname), Some(tail @ (":" | "," | ""))) = (source_nickname, message_tail)
            {
                sender.send_privmsg(target, nickname.to_owned() + tail)?;
            }
        }

        Ok(())
    }
}
