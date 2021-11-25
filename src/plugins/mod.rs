use irc::client::prelude::*;

pub use no_homo_plugin::NoHomoPlugin;

mod no_homo_plugin;

pub trait Plugin {
    fn matches(&self, msg: &str) -> bool;
    fn call(&self, client: &Client, target: &str, msg: &str) -> irc::error::Result<()>;
}
