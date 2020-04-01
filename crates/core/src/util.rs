use std::env;

pub(crate) fn cmd_arg<V: ToString>(k: &str, v: V) -> String {
	format!("{}={}", k, escape_property(v))
}

pub(crate) fn escape_data<D: ToString>(data: D) -> String {
	data.to_string()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
}

pub(crate) fn escape_property<P: ToString>(prop: P) -> String {
	prop.to_string()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
		.replace(':', "%3A")
		.replace(',', "%2C")
}

pub(crate) fn var_from_name<K: ToString>(
	prefix: &str,
	name: K,
) -> Result<String, env::VarError> {
	let suffix = name.to_string().replace(' ', "_").to_uppercase();
	let key = format!("{}_{}", prefix, suffix);

	env::var(key)
}
