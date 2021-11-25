use irc::client::prelude::*;

pub(crate) use dorito_plugin::DoritoPlugin;
pub(crate) use lucario_plugin::LucarioPlugin;
pub(crate) use nohomo_plugin::NohomoPlugin;

mod dorito_plugin;
mod lucario_plugin;
mod nohomo_plugin;

pub(crate) trait Plugin {
    fn matches(&self, msg: &str) -> bool;
    fn call(&self, client: &Client, target: &str, msg: &str) -> irc::error::Result<()>;
}
