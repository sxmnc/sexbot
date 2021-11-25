macro_rules! register {
	() => {
        {
            use crate::plugins::Plugin;
		    Vec::<Box<dyn Plugin>>::new()
        }
	};
    ($($plugin:ident),+$(,)?) => {
        {
            use crate::plugins::Plugin;
            let mut plugins = Vec::<Box<dyn Plugin>>::new();
            $(plugins.push(Box::new($plugin::new()));)*
            plugins
        }
    };
}

pub(crate) use register;
