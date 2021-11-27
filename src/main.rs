use futures::prelude::*;
use irc::client::prelude::*;

use crate::macros::*;
use crate::plugins::*;

static CONFIG_PATH: &str = "config.toml";

mod macros;
mod plugins;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let config = Config::load(CONFIG_PATH)?;

    let plugins = register! {
        config,
        BekePlugin,
        DoritoPlugin,
        LucarioPlugin,
        NohomoPlugin,
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    loop {
        let message = stream.select_next_some().await?;

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
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
