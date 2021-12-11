use irc::client::prelude::*;

pub use beke_plugin::BekePlugin;
pub use dorito_plugin::DoritoPlugin;
pub use help_plugin::HelpPlugin;
pub use lmgtfy_plugin::LmgtfyPlugin;
pub use lucario_plugin::LucarioPlugin;
pub use math_plugin::MathPlugin;
pub use metrics_plugin::MetricsPlugin;
pub use nohomo_plugin::NohomoPlugin;
pub use pleurniche_plugin::PleurnichePlugin;
pub use quotes_plugin::QuotesPlugin;
pub use reply_plugin::ReplyPlugin;

pub mod beke_plugin;
pub mod dorito_plugin;
pub mod help_plugin;
pub mod lmgtfy_plugin;
pub mod lucario_plugin;
pub mod math_plugin;
pub mod metrics_plugin;
pub mod nohomo_plugin;
pub mod pleurniche_plugin;
pub mod quotes_plugin;
pub mod reply_plugin;

pub trait Plugin {
    fn configure(&mut self, _config: &Config) {}
    fn matches(&self, message: &Message) -> bool;
    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()>;
}

pub type Plugins = Vec<Box<dyn Plugin>>;
