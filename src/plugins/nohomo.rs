use irc::client::prelude::*;

use crate::macros::*;

#[derive(Default)]
pub struct NohomoPlugin {
    trigger: String,
    message: String,
}

impl super::Plugin for NohomoPlugin {
    fn configure(&mut self, config: &Config) {
        self.trigger = get_required!(config, "nohomo_trigger");
        self.message = get_required!(config, "nohomo_message");
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.contains(&self.trigger)
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let sender = client.sender();
            sender.send_privmsg(target, &self.message)?;
        }

        Ok(())
    }
}
