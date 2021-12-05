use futures::prelude::*;
use irc::client::prelude::*;

use crate::plugins::*;

static CONFIG_PATH: &str = "config.toml";

mod macros;
mod plugins;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let mut ext = Plugins::new();
    register!(ext, BekePlugin);
    register!(ext, DoritoPlugin);
    register!(ext, HelpPlugin);
    register!(ext, LmgtfyPlugin);
    register!(ext, LucarioPlugin);
    register!(ext, PleurnichePlugin);
    register!(ext, NohomoPlugin);
    register!(ext, ReplyPlugin);

    let config = Config::load(CONFIG_PATH)?;
    for plugin in &mut ext {
        plugin.configure(&config);
    }

    let mut metrics_plugin = MetricsPlugin::new();
    metrics_plugin.configure(&config);
    metrics_plugin.plugin_count = ext.len() + 1;
    ext.push(Box::new(metrics_plugin));

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
