use irc::client::prelude::*;

pub(crate) type Plugins = Vec<Box<dyn Plugin>>;

pub(crate) trait Plugin {
    fn matches(&self, msg: &str) -> bool;
    fn call(
        &self,
        client: &Client,
        target: &str,
        msg: &str,
        prefix: String,
    ) -> irc::error::Result<()>;
}
