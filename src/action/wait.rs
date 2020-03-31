use std::time::Duration;

use actions_toolkit::prelude::*;
use anyhow::{Context, Result};

pub fn wait() -> Result<()> {
	let ms = input("milliseconds")
		.context("milliseconds input required")?
		.parse()
		.context("invalid milliseconds")?;

	let ms = Duration::from_millis(ms);

	std::thread::sleep(ms);

	Ok(())
}
