use std::env;
use std::ffi::OsStr;

use uuid::Uuid;

const PATH_VAR: &str = "PATH";

#[cfg(not(windows))]
const DELIMITER: &str = ":";

#[cfg(windows)]
const DELIMITER: &str = ";";

fn escape_data<D: AsRef<str>>(data: D) -> String {
	data.as_ref()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
}

fn escape_property<P: AsRef<str>>(prop: P) -> String {
	prop.as_ref()
		.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
		.replace(':', "%3A")
		.replace(',', "%2C")
}

fn issue<V: AsRef<str>>(k: &str, v: V) {
	println!("::{}::{}", k, escape_data(v));
}

fn issue_named<K: AsRef<str>, V: AsRef<str>>(name: &str, k: K, v: V) {
	println!("::{} name={}::{}", name, escape_property(k), escape_data(v),);
}

fn var_from_name<K: AsRef<str>>(
	prefix: &str,
	name: K,
) -> Result<String, env::VarError> {
	let suffix = name.as_ref().replace(' ', "_").to_uppercase();
	let key = format!("{}_{}", prefix, suffix);

	env::var(key)
}

pub fn input<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	var_from_name("INPUT", name)
}

pub fn set_output<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	issue_named("set-output", k, v);
}

pub fn export_variable<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	env::set_var(k.as_ref(), v.as_ref());
	issue_named("set-env", k, v);
}

pub fn set_secret<V: AsRef<str>>(v: V) {
	issue("add-mask", v);
}

pub fn add_path<P: AsRef<str>>(v: P) {
	let v = v.as_ref();

	issue("add-path", v);

	let path = if let Some(mut path) = env::var_os(PATH_VAR) {
		path.push(DELIMITER);
		path.push(v);

		path
	} else {
		v.into()
	};

	env::set_var(PATH_VAR, path);
}

pub fn is_debug() -> bool {
	env::var_os("RUNNER_DEBUG").as_deref() == Some(OsStr::new("1"))
}

// TODO: These should handle file, line, col
pub fn debug<M: AsRef<str>>(message: M) {
	issue("debug", message);
}

pub fn error<M: AsRef<str>>(message: M) {
	issue("error", message);
}

pub fn warning<M: AsRef<str>>(message: M) {
	issue("warning", message);
}

pub fn info<M: AsRef<str>>(message: M) {
	println!("{}", message.as_ref());
}

pub fn save_state<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	issue_named("save-state", k, v);
}

pub fn get_state<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	var_from_name("STATE", name)
}

pub fn stop_logging<F, T>(f: F) -> T
where
	F: FnOnce() -> T,
{
	let token = Uuid::new_v4().to_string();

	issue("stop-commands", &token);

	let result = f();

	issue(&token, "");

	result
}

#[test]
fn test_input_exists() {
	let val = "some expected output";

	env::set_var("INPUT_TEST_A_LONG_VAR", val);

	assert_eq!(input("test_a_long_var".to_owned()), Ok(val.to_owned()));
	assert_eq!(input("test a long var"), Ok(val.to_owned()));
	assert_eq!(input("test a_LONG var"), Ok(val.to_owned()));
}

#[test]
fn test_input_does_not_exist() {
	assert_eq!(input("nope"), Err(env::VarError::NotPresent));
}
