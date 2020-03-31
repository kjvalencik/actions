use std::env;
use std::ffi::OsStr;
use std::io;

pub use crate::core::*;
pub use crate::logger::*;

mod core;
mod logger;
mod util;

trait AssertStdout<T> {
	fn assert(self) -> T;
}

impl<T> AssertStdout<T> for io::Result<T> {
	fn assert(self) -> T {
		match self {
			Ok(v) => v,
			Err(e) => panic!("failed printing to stdout: {}", e),
		}
	}
}

pub fn input<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	util::var_from_name("INPUT", name)
}

pub fn set_output<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().set_output(k, v).assert();
}

pub fn export_variable<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().export_variable(k, v).assert();
}

pub fn set_secret<V: AsRef<str>>(v: V) {
	Core::new().set_secret(v).assert();
}

pub fn add_path<P: AsRef<str>>(v: P) {
	Core::new().add_path(v).assert();
}

pub fn save_state<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().save_state(k, v).assert();
}

pub fn get_state<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	util::var_from_name("STATE", name)
}

pub fn stop_logging<F, T>(f: F) -> T
where
	F: FnOnce() -> T,
{
	Core::new().stop_logging(f).assert()
}

pub fn is_debug() -> bool {
	env::var_os("RUNNER_DEBUG").as_deref() == Some(OsStr::new("1"))
}

pub fn debug<M: AsRef<str>>(message: M) {
	Core::new().debug(message).assert();
}

pub fn error<M: AsRef<str>>(message: M) {
	Core::new().error(message).assert();
}

pub fn warning<M: AsRef<str>>(message: M) {
	Core::new().warning(message).assert();
}

pub fn log_message<M: AsRef<str>>(level: LogLevel, message: M) {
	Core::new().log_message(level, message).assert();
}

pub fn log<M: AsRef<str>>(level: LogLevel, log: Log<M>) {
	Core::new().log(level, log).assert();
}

pub fn log_debug<M: AsRef<str>>(log: Log<M>) {
	Core::new().log_debug(log).assert();
}

pub fn log_error<M: AsRef<str>>(log: Log<M>) {
	Core::new().log_error(log).assert();
}

pub fn log_warning<M: AsRef<str>>(log: Log<M>) {
	Core::new().log_warning(log).assert();
}
