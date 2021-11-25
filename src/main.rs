use futures::prelude::*;
use irc::client::prelude::*;

use plugins::*;

mod plugins;

static CONFIG_PATH: &str = "config.toml";

fn load_plugins() -> Vec<Box<dyn Plugin>> {
    let mut plugins = Vec::<Box<dyn Plugin>>::new();
    plugins.push(Box::new(DoritoPlugin::new()));
    plugins.push(Box::new(NoHomoPlugin::new()));
    plugins
}

async fn load_client(config_path: &str) -> irc::error::Result<Client> {
    let config = Config::load(config_path)?;
    let client = Client::from_config(config).await?;
    client.identify()?;
    Ok(client)
}

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let plugins = load_plugins();
    let mut client = load_client(CONFIG_PATH).await?;
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

//     "$beke" => {}
//     "$lucario" => {}
//     "$hash" => {}
//     "$help" => {}
//     "$lmgtfy" | "$snob" | "$rtfm" => {}
//     "$math" => {}
//     "$pleurniche" => {}
//     "$quote" | "$quotes" => {}
//     "$roll" => {}
//     "$select" => {}
//     "$somafm" => {}
//     "$translate" => {}
//     "$typist" => {}
//     "$ud" => {}
//     "$vote" => {}
//     "$weather" => {}
//     _ if msg.starts_with(client.current_nickname()) => {}
//     _ if msg.starts_with("http") => {}
