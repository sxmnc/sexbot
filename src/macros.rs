macro_rules! register {
    ($config:ident, $($plugin:ident),+ $(,)?) => {
		{
			use crate::plugins::Plugin;
			let mut plugins = Vec::<Box<dyn Plugin>>::new();
			$(plugins.push(Box::new($plugin::new(&$config)));)*
			plugins
		}
	};
}

pub(crate) use register;
