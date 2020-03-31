use std::env;
use std::io::{self, Write};

use crate::logger::{Log, LogLevel};
use crate::util;

use uuid::Uuid;

const PATH_VAR: &str = "PATH";

#[cfg(not(windows))]
const DELIMITER: &str = ":";

#[cfg(windows)]
const DELIMITER: &str = ";";

pub struct Core<W> {
	out: W,
}

impl Default for Core<std::io::Stdout> {
	fn default() -> Self {
		Self {
			out: std::io::stdout(),
		}
	}
}

impl Core<std::io::Stdout> {
	pub fn new() -> Self {
		Default::default()
	}
}

impl<W> Core<W>
where
	W: Write,
{
	fn issue<V: AsRef<str>>(&mut self, k: &str, v: V) -> io::Result<()> {
		writeln!(self.out, "::{}::{}", k, util::escape_data(v))
	}

	fn issue_named<K: AsRef<str>, V: AsRef<str>>(
		&mut self,
		name: &str,
		k: K,
		v: V,
	) -> io::Result<()> {
		writeln!(
			self.out,
			"::{} {}::{}",
			name,
			util::cmd_arg("name", k),
			util::escape_data(v),
		)
	}

	pub fn input<K: AsRef<str>>(
		_: &Self,
		name: K,
	) -> Result<String, env::VarError> {
		crate::input(name)
	}

	pub fn set_output<K: AsRef<str>, V: AsRef<str>>(
		&mut self,
		k: K,
		v: V,
	) -> io::Result<()> {
		self.issue_named("set-output", k, v)
	}

	pub fn export_variable<K: AsRef<str>, V: AsRef<str>>(
		&mut self,
		k: K,
		v: V,
	) -> io::Result<()> {
		// TODO: Move the side effect to a struct member
		env::set_var(k.as_ref(), v.as_ref());

		self.issue_named("set-env", k, v)
	}

	pub fn set_secret<V: AsRef<str>>(&mut self, v: V) -> io::Result<()> {
		self.issue("add-mask", v)
	}

	pub fn add_path<P: AsRef<str>>(&mut self, v: P) -> io::Result<()> {
		let v = v.as_ref();

		self.issue("add-path", v)?;

		// TODO: Move the side effect to a struct member
		let path = if let Some(mut path) = env::var_os(PATH_VAR) {
			path.push(DELIMITER);
			path.push(v);

			path
		} else {
			v.into()
		};

		env::set_var(PATH_VAR, path);

		Ok(())
	}

	pub fn save_state<K: AsRef<str>, V: AsRef<str>>(
		&mut self,
		k: K,
		v: V,
	) -> io::Result<()> {
		self.issue_named("save-state", k, v)
	}

	pub fn get_state<K: AsRef<str>>(
		_: &Self,
		name: K,
	) -> Result<String, env::VarError> {
		crate::get_state(name)
	}

	pub fn stop_logging<F, T>(&mut self, f: F) -> io::Result<T>
	where
		F: FnOnce() -> T,
	{
		let token = Uuid::new_v4().to_string();

		self.issue("stop-commands", &token)?;

		let result = f();

		self.issue(&token, "")?;

		Ok(result)
	}

	pub fn is_debug(_: &Self) -> bool {
		crate::is_debug()
	}

	pub fn log_message<M: AsRef<str>>(
		&mut self,
		level: LogLevel,
		message: M,
	) -> io::Result<()> {
		self.issue(level.as_ref(), message)
	}

	pub fn debug<M: AsRef<str>>(&mut self, message: M) -> io::Result<()> {
		self.log_message(LogLevel::Debug, message)
	}

	pub fn error<M: AsRef<str>>(&mut self, message: M) -> io::Result<()> {
		self.log_message(LogLevel::Error, message)
	}

	pub fn warning<M: AsRef<str>>(&mut self, message: M) -> io::Result<()> {
		self.log_message(LogLevel::Warning, message)
	}

	pub fn log<M: AsRef<str>>(
		&mut self,
		level: LogLevel,
		log: Log<M>,
	) -> io::Result<()> {
		writeln!(self.out, "::{}{}", level.as_ref(), log)
	}

	pub fn log_debug<M: AsRef<str>>(&mut self, log: Log<M>) -> io::Result<()> {
		self.log(LogLevel::Debug, log)
	}

	pub fn log_error<M: AsRef<str>>(&mut self, log: Log<M>) -> io::Result<()> {
		self.log(LogLevel::Error, log)
	}

	pub fn log_warning<M: AsRef<str>>(
		&mut self,
		log: Log<M>,
	) -> io::Result<()> {
		self.log(LogLevel::Warning, log)
	}
}
