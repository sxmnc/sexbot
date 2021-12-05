#[macro_export]
macro_rules! get_required {
    ($config:ident, $key:literal) => {
        $config.get_option($key).unwrap().to_owned()
    };
}

#[macro_export]
macro_rules! register {
    ($ext:ident, $plugin:ident) => {
        $ext.push(Box::new($plugin::new()))
    };
}

pub use get_required;
