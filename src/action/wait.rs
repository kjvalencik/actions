use std::time::Duration;

use anyhow::{Context, Result};

fn input(name: &str) -> Result<String, std::env::VarError> {
	let suffix = name.replace(' ', "_").to_uppercase();
	let key = format!("INPUT_{}", suffix);

	std::env::var(key)
}

pub fn wait() -> Result<()> {
	let ms = input("milliseconds")
		.context("milliseconds input required")?
		.parse()
		.context("invalid milliseconds")?;

	let ms = Duration::from_millis(ms);

	std::thread::sleep(ms);

	Ok(())
}
