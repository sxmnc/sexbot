use irc::client::prelude::*;

static TRIGGER: &str = "$beke";
static BEKE_NICKNAME: &str = "KwameBeke";
static RESPONSE: &str = "Hé hé hé...";

#[derive(Default)]
pub struct Beke {
    nickname: String,
}

impl super::Plugin for Beke {
    fn configure(&mut self, config: &Config) {
        self.nickname = config.nickname.as_ref().unwrap().to_owned();
    }

    fn matches(&self, msg: &str) -> bool {
        msg == TRIGGER
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send(Command::NICK(BEKE_NICKNAME.to_owned()))?;
        sender.send_privmsg(target, RESPONSE)?;
        sender.send(Command::NICK(self.nickname.to_owned()))?;
        Ok(())
    }
}
