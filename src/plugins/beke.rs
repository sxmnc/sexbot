use irc::client::prelude::*;

use crate::prelude::*;

pub struct BekePlugin {
    nickname: String,
    trigger: String,
    temp_nickname: String,
    message: String,
}

impl BekePlugin {
    pub fn new(config: &Config) -> BekePlugin {
        BekePlugin {
            nickname: config.nickname().unwrap().to_owned(),
            trigger: get_required!(config, "beke_trigger"),
            temp_nickname: get_required!(config, "beke_temp_nickname"),
            message: get_required!(config, "beke_message"),
        }
    }
}

impl Plugin for BekePlugin {
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
