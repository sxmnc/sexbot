use chrono::{DateTime, Utc};
use irc::client::prelude::*;

use crate::macros::*;

#[derive(Default)]
pub struct MetricsPlugin {
    start_time: Option<DateTime<Utc>>,
    pub plugin_count: usize,
    trigger: String,
}

impl super::Plugin for MetricsPlugin {
    fn configure(&mut self, config: &Config) {
        self.start_time = Some(Utc::now());
        self.trigger = get_required!(config, "metrics_trigger").to_lowercase();
    }

    fn matches(&self, message: &Message) -> bool {
        if let Command::PRIVMSG(ref _target, ref msg) = message.command {
            msg.trim().to_lowercase() == self.trigger
        } else {
            false
        }
    }

    fn call(&self, client: &Client, message: &Message) -> irc::error::Result<()> {
        if let Command::PRIVMSG(ref target, ref _msg) = message.command {
            let sender = client.sender();

            let uptime = Utc::now() - self.start_time.unwrap();
            let num_seconds = uptime.num_seconds();

            let seconds = num_seconds % 60;
            let minutes = num_seconds / 60 % 60;
            let hours = num_seconds / 60 / 60 % 24;
            let days = num_seconds / 60 / 60 / 24;

            sender.send_privmsg(
                target,
                format!(
                    "{} plugins loaded; up {} days, {} hours, {} minutes, {} seconds",
                    self.plugin_count, days, hours, minutes, seconds,
                ),
            )?;
        }

        Ok(())
    }
}
