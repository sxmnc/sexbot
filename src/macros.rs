#[macro_export]
macro_rules! get_required {
    ($config:expr, $key:expr) => {
        $config.get_option($key).unwrap().to_owned()
    };
}

#[macro_export]
macro_rules! register {
    ($config:expr, $($plugin:ident),+ $(,)?) => {
		{
			use crate::plugins::Plugin;
			let mut plugins = Vec::<Box<dyn Plugin>>::new();
			$(
				plugins.push(Box::new($plugin::new($config)));
			)*
			plugins
		}
	};
}

pub(crate) use get_required;
