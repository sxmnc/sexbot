use irc::client::prelude::*;

use crate::macros::*;

#[derive(Default)]
pub struct BekePlugin {
    nickname: String,
    trigger: String,
    temp_nickname: String,
    message: String,
}

impl super::Plugin for BekePlugin {
    fn configure(&mut self, config: &Config) {
        self.nickname = config.nickname().unwrap().to_owned();
        self.trigger = get_required!(config, "beke_trigger").to_lowercase();

        self.temp_nickname = get_required!(config, "beke_temp_nickname");
        self.message = get_required!(config, "beke_message");
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.trim().to_lowercase() == self.trigger
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let sender = client.sender();
            sender.send(Command::NICK(self.temp_nickname.to_owned()))?;
            sender.send_privmsg(target, &self.message)?;
            sender.send(Command::NICK(self.nickname.to_owned()))?;
        }

        Ok(())
    }
}
