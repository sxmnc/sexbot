use irc::client::prelude::*;

static TRIGGER: &str = "<3";
static RESPONSE: &str = "#nohomo";

pub struct NohomoPlugin;

impl NohomoPlugin {
    pub fn new() -> NohomoPlugin {
        NohomoPlugin
    }
}

impl super::Plugin for NohomoPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.contains(TRIGGER)
    }

    fn call(&self, client: &Client, target: &str, _msg: &str) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, RESPONSE)?;
        Ok(())
    }
}
