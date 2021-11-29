use futures::prelude::*;
use irc::client::prelude::*;

use crate::prelude::*;

static CONFIG_PATH: &str = "config.toml";

mod macros;
mod plugins;
mod prelude;

fn load_plugins(config: &Config) -> Plugins {
    build_plugins! {
        config,
        BekePlugin,
        DoritoPlugin,
        HelpPlugin,
        LucarioPlugin,
        NohomoPlugin,
    }
}

async fn main_setup() -> irc::error::Result<(Client, Plugins)> {
    let config = Config::load(CONFIG_PATH)?;
    let mut plugins = load_plugins(&config);
    plugins.push(Box::new(MetricsPlugin::new(&config, &plugins)));

    let client = Client::from_config(config).await?;
    client.identify()?;

    Ok((client, plugins))
}

async fn main_loop(mut client: Client, plugins: Plugins) -> irc::error::Result<()> {
    let mut stream = client.stream()?;
    loop {
        let message = stream.select_next_some().await?;
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let msg = msg.trim();
                for plugin in &plugins {
                    if plugin.matches(msg) {
                        plugin.call(&client, target, msg)?;
                        break;
                    }
                }
            }
            _ => (),
        }
    }
}

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let (client, plugins) = main_setup().await?;
    main_loop(client, plugins).await
}
