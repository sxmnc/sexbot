use irc::client::prelude::*;

pub struct LucarioPlugin {
    trigger: String,
    message: String,
}

impl LucarioPlugin {
    pub fn new(config: &Config) -> LucarioPlugin {
        LucarioPlugin {
            trigger: config.get_option("lucario_trigger").unwrap().to_owned(),
            message: config.get_option("lucario_message").unwrap().to_owned(),
        }
    }
}

impl super::Plugin for LucarioPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg == self.trigger
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, &self.message)?;
        Ok(())
    }
}
