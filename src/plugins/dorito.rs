use irc::client::prelude::*;
use rand::Rng;

use crate::macros::*;

#[derive(Default)]
pub struct DoritoPlugin {
    trigger: String,
    message: String,
    false_message: String,
    true_message: String,
    troll_message: String,
}

impl super::Plugin for DoritoPlugin {
    fn configure(&mut self, config: &Config) {
        self.trigger = get_required!(config, "dorito_trigger");
        self.message = get_required!(config, "dorito_message");
        self.false_message = get_required!(config, "dorito_false_message");
        self.true_message = get_required!(config, "dorito_true_message");
        self.troll_message = get_required!(config, "dorito_troll_message");
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.trim() == self.trigger
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let sender = client.sender();
            sender.send_privmsg(target, &self.message)?;

            let mut rng = rand::thread_rng();

            if rng.gen_range(0..=1) == 0 {
                if rng.gen_range(0..=4) == 0 {
                    sender.send_privmsg(target, &self.false_message)?;
                } else {
                    sender.send_privmsg(target, &self.true_message)?;
                }
            }

            if rng.gen_range(0..=1) == 0 {
                sender.send_privmsg(target, &self.troll_message)?;
            }
        }

        Ok(())
    }
}
