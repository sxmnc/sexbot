use irc::client::prelude::*;

pub struct BekePlugin {
    nickname: String,
    trigger: String,
    temp_nickname: String,
    message: String,
}

impl BekePlugin {
    pub fn new(config: &Config) -> BekePlugin {
        BekePlugin {
            nickname: config.nickname.as_ref().unwrap().to_owned(),
            trigger: config.get_option("beke_trigger").unwrap().to_owned(),
            temp_nickname: config.get_option("beke_nickname").unwrap().to_owned(),
            message: config.get_option("beke_message").unwrap().to_owned(),
        }
    }
}

impl super::Plugin for BekePlugin {
    fn matches(&self, msg: &str) -> bool {
        msg == self.trigger
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send(Command::NICK(self.temp_nickname.clone()))?;
        sender.send_privmsg(target, &self.message)?;
        sender.send(Command::NICK(self.nickname.clone()))?;
        Ok(())
    }
}
