#[allow(unused_imports)]
use plugins::*;

mod app;
mod plugins;

static CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let plugins = app::register! {
        DoritoPlugin,
        LucarioPlugin,
        NoHomoPlugin,
    };

    let client = app::load_client(CONFIG_PATH).await?;
    app::main_loop(client, plugins).await
}
