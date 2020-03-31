use std::env;
use std::io::{self, Write};

use crate::logger::{Log, LogLevel};
use crate::util;

use uuid::Uuid;

const PATH_VAR: &str = "PATH";

#[cfg(not(windows))]
pub(crate) const DELIMITER: &str = ":";

#[cfg(windows)]
pub(crate) const DELIMITER: &str = ";";

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

impl<W: Write> From<W> for Core<W> {
	fn from(out: W) -> Self {
		Core { out }
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

	// TODO: Should the API prevent compiling code that will output commands
	// while this is running?
	pub fn stop_logging<F, T>(&mut self, f: F) -> io::Result<T>
	where
		F: FnOnce() -> T,
	{
		// TODO: Allow the to be configurable (helpful for tests)
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

#[cfg(test)]
mod test {
	use std::cell::RefCell;
	use std::env;
	use std::io;
	use std::rc::Rc;

	use crate::core::DELIMITER;
	use crate::*;

	#[derive(Clone)]
	struct TestBuf {
		inner: Rc<RefCell<Vec<u8>>>,
	}

	impl TestBuf {
		fn new() -> Self {
			Self {
				inner: Rc::new(RefCell::new(Vec::new())),
			}
		}

		fn clear(&self) {
			self.inner.borrow_mut().clear();
		}

		fn to_string(&self) -> String {
			String::from_utf8(self.inner.borrow().to_vec()).unwrap()
		}
	}

	impl io::Write for TestBuf {
		fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
			self.inner.borrow_mut().write(buf)
		}

		fn flush(&mut self) -> io::Result<()> {
			self.inner.borrow_mut().flush()
		}
	}

	fn test<F>(expected: &str, f: F)
	where
		F: FnOnce(Core<TestBuf>) -> io::Result<()>,
	{
		let buf = TestBuf::new();

		f(Core::from(buf.clone())).unwrap();

		assert_eq!(buf.to_string(), expected);
	}

	#[test]
	fn set_output() {
		test("::set-output name=greeting::hello\n", |mut core| {
			core.set_output("greeting", "hello")
		});
	}

	#[test]
	fn export_variable() {
		test("::set-env name=greeting::hello\n", |mut core| {
			core.export_variable("greeting", "hello")
		});

		assert_eq!(env::var("greeting").unwrap().as_str(), "hello");
	}

	#[test]
	fn set_secret() {
		test("::add-mask::super secret message\n", |mut core| {
			core.set_secret("super secret message")
		});
	}

	#[test]
	fn add_path() {
		test("::add-path::/this/is/a/test\n", |mut core| {
			core.add_path("/this/is/a/test")
		});

		let path = env::var("PATH").unwrap();
		let last_path = path.split(DELIMITER).last().unwrap();

		assert_eq!(last_path, "/this/is/a/test");
	}

	#[test]
	fn save_state() {
		test("::save-state name=greeting::hello\n", |mut core| {
			core.save_state("greeting", "hello")
		});
	}

	#[test]
	fn stop_logging() {
		let buf = TestBuf::new();
		let mut core = Core::from(buf.clone());
		let mut token = String::new();

		core.stop_logging(|| {
			let output = buf.to_string();

			assert!(output.starts_with("::stop-commands::"));

			token = output.trim().split("::").last().unwrap().to_string();
			buf.clear();
		})
		.unwrap();

		assert_eq!(buf.to_string(), format!("::{}::\n", token));
	}

	#[test]
	fn test_debug() {
		test("::debug::Hello, World!\n", |mut core| {
			core.debug("Hello, World!")
		});
	}

	#[test]
	fn test_error_complex() {
		test(
			"::error file=/test/file.rs,line=5,col=10::hello\n",
			|mut core| {
				core.log_error(Log {
					message: "hello",
					file: Some("/test/file.rs"),
					line: Some(5),
					col: Some(10),
				})
			},
		);
	}

	#[test]
	fn test_warning_omit() {
		test("::warning::hello\n", |mut core| {
			core.log_warning(Log {
				message: "hello",
				..Default::default()
			})
		});
	}
}
