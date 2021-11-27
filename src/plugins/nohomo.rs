use irc::client::prelude::*;

pub struct NohomoPlugin {
    trigger: String,
    message: String,
}

impl NohomoPlugin {
    pub fn new(config: &Config) -> NohomoPlugin {
        NohomoPlugin {
            trigger: config.get_option("nohomo_trigger").unwrap().to_owned(),
            message: config.get_option("nohomo_message").unwrap().to_owned(),
        }
    }
}

impl super::Plugin for NohomoPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.contains(&self.trigger)
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, &self.message)?;
        Ok(())
    }
}
