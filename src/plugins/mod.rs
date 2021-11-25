use irc::client::prelude::*;

pub(crate) use dorito_plugin::DoritoPlugin;
pub(crate) use lucario_plugin::LucarioPlugin;
pub(crate) use no_homo_plugin::NoHomoPlugin;

mod dorito_plugin;
mod lucario_plugin;
mod no_homo_plugin;

pub(crate) trait Plugin {
    fn matches(&self, msg: &str) -> bool;
    fn call(&self, client: &Client, target: &str, msg: &str) -> irc::error::Result<()>;
}
