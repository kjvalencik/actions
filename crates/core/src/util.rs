use std::env;

pub(crate) fn cmd_arg<V: AsRef<str>>(k: &str, v: V) -> String {
	format!("{}={}", k, escape_property(v))
}

pub(crate) fn escape_data<D: AsRef<str>>(data: D) -> String {
	data.as_ref()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
}

pub(crate) fn escape_property<P: AsRef<str>>(prop: P) -> String {
	prop.as_ref()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
		.replace(':', "%3A")
		.replace(',', "%2C")
}

pub(crate) fn var_from_name<K: AsRef<str>>(
	prefix: &str,
	name: K,
) -> Result<String, env::VarError> {
	let suffix = name.as_ref().replace(' ', "_").to_uppercase();
	let key = format!("{}_{}", prefix, suffix);

	env::var(key)
}
