use irc::client::prelude::*;

use crate::prelude::*;

pub struct NohomoPlugin {
    trigger: String,
    message: String,
}

impl NohomoPlugin {
    pub fn new(config: &Config) -> NohomoPlugin {
        NohomoPlugin {
            trigger: get_required!(config, "nohomo_trigger"),
            message: get_required!(config, "nohomo_message"),
        }
    }
}

impl Plugin for NohomoPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.contains(&self.trigger)
    }

    fn call(
        &self,
        client: &Client,
        target: &str,
        _msg: &str,
        _prefix: String,
    ) -> irc::error::Result<()> {
        let sender = client.sender();
        sender.send_privmsg(target, &self.message)?;
        Ok(())
    }
}
