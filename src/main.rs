mod action;

#[derive(Debug)]
enum Command {
	Wait,
}

impl std::str::FromStr for Command {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let cmd = match s {
			"wait" => Command::Wait,
			_ => return Err(()),
		};

		Ok(cmd)
	}
}

fn main() {
	let cmd = match std::env::args().nth(1) {
		Some(cmd) => cmd,
		None => {
			eprintln!("Must provide a command");
			std::process::exit(2);
		}
	};

	let cmd = match cmd.parse::<Command>() {
		Ok(cmd) => cmd,
		Err(_) => {
			eprintln!("Invalid command: {}", cmd);
			std::process::exit(1);
		}
	};

	let result = match cmd {
		Command::Wait => action::wait(),
	};

	if let Err(err) = result {
		eprintln!("{:?}", err);

		std::process::exit(1);
	}
}
