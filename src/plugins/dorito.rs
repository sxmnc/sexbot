use irc::client::prelude::*;
use rand::Rng;

use crate::prelude::*;

pub struct DoritoPlugin {
    trigger: String,
    message: String,
    false_message: String,
    true_message: String,
    troll_message: String,
}

impl DoritoPlugin {
    pub fn new(config: &Config) -> DoritoPlugin {
        DoritoPlugin {
            trigger: get_required!(config, "dorito_trigger"),
            message: get_required!(config, "dorito_message"),
            false_message: get_required!(config, "dorito_false_message"),
            true_message: get_required!(config, "dorito_true_message"),
            troll_message: get_required!(config, "dorito_troll_message"),
        }
    }
}

impl Plugin for DoritoPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg == self.trigger
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
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
        Ok(())
    }
}
