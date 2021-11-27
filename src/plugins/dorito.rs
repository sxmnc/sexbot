use irc::client::prelude::*;
use rand::Rng;

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
            trigger: config.get_option("dorito_trigger").unwrap().to_owned(),
            message: config.get_option("dorito_message").unwrap().to_owned(),
            false_message: config.get_option("dorito_false").unwrap().to_owned(),
            true_message: config.get_option("dorito_true").unwrap().to_owned(),
            troll_message: config.get_option("dorito_troll").unwrap().to_owned(),
        }
    }
}

impl super::Plugin for DoritoPlugin {
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
