use std::fmt;

use crate::util;

#[derive(Debug)]
pub enum LogLevel {
	Debug,
	Error,
	Warning,
}

impl AsRef<str> for LogLevel {
	fn as_ref(&self) -> &str {
		match self {
			LogLevel::Debug => "debug",
			LogLevel::Error => "error",
			LogLevel::Warning => "warning",
		}
	}
}

#[derive(Debug, Default)]
pub struct Log<'f, M> {
	pub message: M,
	pub file: Option<&'f str>,
	pub line: Option<usize>,
	pub col: Option<usize>,
}

impl<'f, M> Log<'f, M> {
	pub fn message(message: M) -> Self {
		Self {
			message,
			file: None,
			line: None,
			col: None,
		}
	}
}

impl<'f, M> fmt::Display for Log<'f, M>
where
	M: ToString,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.file.is_none() && self.line.is_none() && self.col.is_none() {
			return write!(f, "::{}", self.message.to_string());
		}

		let args = vec![
			self.file.map(|f| util::cmd_arg("file", f)),
			self.line.map(|l| util::cmd_arg("line", l.to_string())),
			self.col.map(|c| util::cmd_arg("col", c.to_string())),
		];

		let args = args.into_iter().flatten().collect::<Vec<_>>().join(",");

		write!(f, " {}::{}", args, self.message.to_string())
	}
}
