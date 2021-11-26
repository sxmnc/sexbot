use irc::client::prelude::*;

pub(crate) trait Plugin {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Self::default()
    }

    fn register(ext: &mut Vec<Box<dyn Plugin>>)
    where
        Self: Sized + Default + 'static,
    {
        ext.push(Box::new(Self::new()));
    }

    fn configure(&mut self, _config: &Config) {}

    fn matches(&self, _msg: &str) -> bool {
        false
    }

    fn call(&self, _client: &Client, _target: &str, _msg: &str) -> irc::error::Result<()> {
        Ok(())
    }
}
