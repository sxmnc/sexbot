use chrono::{DateTime, Utc};
use irc::client::prelude::*;

use crate::prelude::*;

pub struct MetricsPlugin {
    start_time: DateTime<Utc>,
    plugin_count: usize,
    trigger: String,
}

impl MetricsPlugin {
    pub(crate) fn new(config: &Config, plugins: &Plugins) -> MetricsPlugin {
        MetricsPlugin {
            start_time: Utc::now(),
            plugin_count: plugins.len() + 1,
            trigger: get_required!(config, "metrics_trigger").to_lowercase(),
        }
    }
}

impl Plugin for MetricsPlugin {
    fn matches(&self, msg: &str) -> bool {
        msg.to_lowercase() == self.trigger
    }

    fn call(
        &self,
        client: &Client,
        target: &str,
        _msg: &str,
        _prefix: String,
    ) -> irc::error::Result<()> {
        let sender = client.sender();

        let uptime = Utc::now() - self.start_time;
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
        Ok(())
    }
}
