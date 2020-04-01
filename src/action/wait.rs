use std::time::Duration;

use actions_toolkit::core;
use anyhow::{Context, Result};

pub fn wait() -> Result<()> {
	let ms = core::input("milliseconds")
		.context("milliseconds input required")?
		.parse()
		.context("invalid milliseconds")?;

	let ms = Duration::from_millis(ms);

	std::thread::sleep(ms);

	Ok(())
}
