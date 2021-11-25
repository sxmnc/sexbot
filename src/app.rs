use futures::prelude::*;
use irc::client::prelude::*;

use crate::plugins::Plugin;

pub(crate) use crate::macros::register;

pub(crate) async fn load_client(config_path: &str) -> irc::error::Result<Client> {
    let config = Config::load(config_path)?;
    let client = Client::from_config(config).await?;
    client.identify()?;
    Ok(client)
}

pub(crate) async fn main_loop(
    mut client: Client,
    plugins: Vec<Box<dyn Plugin>>,
) -> irc::error::Result<()> {
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
