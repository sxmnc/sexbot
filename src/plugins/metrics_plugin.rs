use chrono::{DateTime, Utc};
use irc::client::prelude::*;
use serde_derive::Deserialize;

use crate::macros::*;

#[derive(Deserialize)]
pub struct MetricsPlugin {
    #[serde(skip_deserializing)]
    start_time: Option<DateTime<Utc>>,
    plugin_count: Option<usize>,
    triggers: Vec<String>,
}

impl MetricsPlugin {
    pub fn new() -> MetricsPlugin {
        from_config!("config/plugins/metrics_config.toml")
    }

    pub fn set_plugin_count(&mut self, plugin_count: usize) {
        self.plugin_count = Some(plugin_count);
    }
}

impl super::Plugin for MetricsPlugin {
    fn configure(&mut self, _config: &Config) {
        self.start_time = Some(Utc::now());
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            self.triggers.contains(&msg.trim().to_lowercase())
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let plugin_count = self.plugin_count.unwrap();
            let uptime = Utc::now() - self.start_time.unwrap();
            let num_seconds = uptime.num_seconds();

            let seconds = num_seconds % 60;
            let minutes = num_seconds / 60 % 60;
            let hours = num_seconds / 60 / 60 % 24;
            let days = num_seconds / 60 / 60 / 24;

            client.send_privmsg(
                target,
                format!(
                    "{} plugins loaded; up {} days, {} hours, {} minutes, {} seconds",
                    plugin_count, days, hours, minutes, seconds,
                ),
            )?;
        }

        Ok(())
    }
}
