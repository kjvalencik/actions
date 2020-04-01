# actions-core

Core functions for inputs, outputs, logging, setting environment variables,
and masking secrets.

## Example

```rust
use std::time::Duration;

use actions_core as core;
use anyhow::{Context, Result};

pub fn main() {
	let ms = core::input("milliseconds")
		.expect("milliseconds input required")?
		.parse()
		.expect("invalid milliseconds")?;

	let ms = Duration::from_millis(ms);

	std::thread::sleep(ms);

	core::set_output("greeting", "Hello, World!");
}
```
