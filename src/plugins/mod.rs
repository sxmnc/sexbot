use irc::client::prelude::*;

pub use beke::BekePlugin;
pub use dorito::DoritoPlugin;
pub use help::HelpPlugin;
pub use lmgtfy::LmgtfyPlugin;
pub use lucario::LucarioPlugin;
pub use metrics::MetricsPlugin;
pub use nohomo::NohomoPlugin;
pub use pleurniche::PleurnichePlugin;
pub use reply::ReplyPlugin;

pub mod beke;
pub mod dorito;
pub mod help;
pub mod lmgtfy;
pub mod lucario;
pub mod metrics;
pub mod nohomo;
pub mod pleurniche;
pub mod reply;

pub trait Plugin {
    fn new() -> Self
    where
        Self: Default,
    {
        Default::default()
    }
    fn configure(&mut self, config: &Config);
    fn matches(&self, message: &Message) -> bool;
    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()>;
}

pub type Plugins = Vec<Box<dyn Plugin>>;
