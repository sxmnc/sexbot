use irc::client::prelude::*;

pub use beke_plugin::BekePlugin;
pub use coinflip_plugin::CoinflipPlugin;
pub use dorito_plugin::DoritoPlugin;
pub use help_plugin::HelpPlugin;
pub use link_plugin::LinkPlugin;
pub use lmgtfy_plugin::LmgtfyPlugin;
pub use lucario_plugin::LucarioPlugin;
pub use math_plugin::MathPlugin;
pub use metrics_plugin::MetricsPlugin;
pub use nohomo_plugin::NohomoPlugin;
pub use pleurniche_plugin::PleurnichePlugin;
pub use quotes_plugin::QuotesPlugin;
pub use reply_plugin::ReplyPlugin;
pub use roll_plugin::RollPlugin;
pub use translate_plugin::TranslatePlugin;
pub use ud_plugin::UdPlugin;

pub mod beke_plugin;
pub mod coinflip_plugin;
pub mod dorito_plugin;
pub mod help_plugin;
pub mod link_plugin;
pub mod lmgtfy_plugin;
pub mod lucario_plugin;
pub mod math_plugin;
pub mod metrics_plugin;
pub mod nohomo_plugin;
pub mod pleurniche_plugin;
pub mod quotes_plugin;
pub mod reply_plugin;
pub mod roll_plugin;
pub mod translate_plugin;
pub mod ud_plugin;

#[async_trait]
pub trait Plugin {
    fn configure(&mut self, _config: &Config) {}
    fn matches(&self, message: &Message) -> bool;
    async fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()>;
}

pub type Plugins = Vec<Box<dyn Plugin>>;
