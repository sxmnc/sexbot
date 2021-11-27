use irc::client::prelude::*;

pub(crate) trait Plugin {
    fn matches(&self, msg: &str) -> bool;
    fn call(&self, client: &Client, target: &str, msg: &str) -> irc::error::Result<()>;
}
