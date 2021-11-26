use futures::prelude::*;
use irc::client::prelude::*;

use plugins::*;

static CONFIG_PATH: &str = "config.toml";

mod plugins;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let mut ext = Vec::new();
    Beke::register(&mut ext);
    Dorito::register(&mut ext);
    Lucario::register(&mut ext);
    Nohomo::register(&mut ext);

    let config = Config::load(CONFIG_PATH)?;
    for plugin in &mut ext {
        plugin.configure(&config);
    }

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    loop {
        let message = stream.select_next_some().await?;

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                for plugin in &ext {
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
