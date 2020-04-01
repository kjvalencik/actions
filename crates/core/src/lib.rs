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

/// Get an action's input parameter.
///
/// ```
/// use actions_core as core;
///
/// # std::env::set_var("INPUT_MILLISECONDS", "1000");
/// let ms: u32 = core::input("milliseconds")
/// 	.expect("Failed to get milliseconds")
/// 	.parse()
/// 	.expect("Failed to parse milliseconds");
/// ```
pub fn input<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	util::var_from_name("INPUT", name)
}

/// Sets an action's output parameter.
///
/// ```
/// use actions_core as core;
///
/// let count = 5;
///
/// core::set_output("count", 5.to_string());
/// ```
pub fn set_output<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().set_output(k, v).assert();
}

/// Creates or updates an environment variable for any actions running next
/// in a job. Environment variables are immediately set and available to the
/// currently running action. Environment variables are case-sensitive and
/// you can include punctuation.
///
/// ```
/// use actions_core as core;
///
/// core::set_env("MY_GREETING", "hello");
///
/// assert_eq!(
/// 	std::env::var_os("MY_GREETING").as_deref(),
/// 	Some(std::ffi::OsStr::new("hello")),
/// );
/// ```
pub fn set_env<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().set_env(k, v).assert();
}

/// Masking a value prevents a string or variable from being printed in the
/// log. Each masked word separated by whitespace is replaced with the `*`
/// character.
///
/// ```
/// use actions_core as core;
///
/// core::add_mask("supersecret");
/// ```
pub fn add_mask<V: AsRef<str>>(v: V) {
	Core::new().add_mask(v).assert();
}

/// Appends a directory to the system PATH variable for all subsequent
/// actions in the current job as well as the currently running action.
///
/// ```
/// use actions_core as core;
///
/// core::add_path("/opt/my-app/bin");
/// ```
pub fn add_path<P: AsRef<str>>(v: P) {
	Core::new().add_path(v).assert();
}

/// Similar to `set_output`, but, shares data from a wrapper action.
///
/// ```
/// use actions_core as core;
///
/// core::save_state("my_greeting", "hello");
/// ```
pub fn save_state<K: AsRef<str>, V: AsRef<str>>(k: K, v: V) {
	Core::new().save_state(k, v).assert();
}

/// Similar to `input`, but, gets data shared from a wrapper action.
///
/// ```
/// use actions_core as core;
///
/// let greeting = core::state("my_greeting")
/// 	.unwrap_or_else(|_| "hello".to_owned());
/// ```
pub fn state<K: AsRef<str>>(name: K) -> Result<String, env::VarError> {
	util::var_from_name("STATE", name)
}

/// Stops processing workflow commands while the provided function runs. A
/// token is randomly generated and used to re-enable commands after
/// completion.
///
/// ```
/// use actions_core as core;
///
/// core::stop_logging(|| {
/// 	println!("::set-env name=ignored::value");
/// });
/// ```
pub fn stop_logging<F, T>(f: F) -> T
where
	F: FnOnce() -> T,
{
	Core::new().stop_logging(f).assert()
}

/// Returns `true` if debugging is enabled Action debugging may be enabled
/// by setting a `ACTION_STEP_DEBUG` secret to `true` in the repo.
///
/// ```
/// use actions_core as core;
///
/// let is_debug = core::is_debug();
/// ```
pub fn is_debug() -> bool {
	env::var_os("RUNNER_DEBUG").as_deref() == Some(OsStr::new("1"))
}

/// Prints a debug message to the log. Action debugging may be enabled by
/// setting a `ACTION_STEP_DEBUG` secret to `true` in the repo. You can
/// optionally provide a `file`, `line` and `col` with the `log_error`
/// function.
///
/// ```
/// use actions_core as core;
///
/// core::debug("shaving a yak");
/// ```
pub fn debug<M: AsRef<str>>(message: M) {
	Core::new().debug(message).assert();
}

/// Prints an error message to the log. You can optionally provide a `file`,
/// `line` and `col` with the `log_error` function.
///
/// ```
/// use actions_core as core;
///
/// core::error("shaving a yak");
/// ```
pub fn error<M: AsRef<str>>(message: M) {
	Core::new().error(message).assert();
}

/// Prints a warning message to the log. You can optionally provide a `file`,
/// `line` and `col` with the `log_warning` function.
///
/// ```
/// use actions_core as core;
///
/// core::warning("shaving a yak");
/// ```
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
