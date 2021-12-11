use futures::prelude::*;
use irc::client::prelude::*;

use crate::plugins::*;

static CONFIG_PATH: &str = "config/client_config.toml";

mod macros;
mod plugins;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let mut ext = register_plugins! {
        BekePlugin,
        DoritoPlugin,
        HelpPlugin,
        LmgtfyPlugin,
        LucarioPlugin,
        MathPlugin,
        PleurnichePlugin,
        QuotesPlugin,
        NohomoPlugin,
        ReplyPlugin,
    };

    let mut metrics = MetricsPlugin::new();
    metrics.set_plugin_count(ext.len() + 1);
    ext.push(Box::new(metrics));

    let config = Config::load(CONFIG_PATH)?;
    for plugin in &mut ext {
        plugin.configure(&config);
    }

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    loop {
        let message = stream.select_next_some().await?;
        for plugin in &ext {
            if plugin.matches(&message) {
                plugin.call(&client, &message)?;
                break;
            }
        }
    }
}
