#[macro_export]
macro_rules! from_config {
    ($path:literal) => {{
        let data =
            std::fs::read_to_string($path).unwrap_or_else(|_| panic!("\"{}\" not found", $path));
        toml::from_str(&data).unwrap()
    }};
}

#[macro_export]
macro_rules! register_plugins {
    ($($plugin:ident),+ $(,)?) => {
        {
            let mut ext = crate::plugins::Plugins::new();
            $(
                ext.push(Box::new($plugin::new()));
            )+
            ext
        }
    };
}

pub use from_config;
