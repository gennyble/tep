mod cli;
mod tep;

use cli::CliArgs;
use tep::Tep;

fn main() -> eyre::Result<()> {
	let cli = match CliArgs::parse()? {
		None => return Ok(()),
		Some(c) => c,
	};

	let tep = Tep::file(cli.input_path).unwrap();
	tep.save_as_png(cli.output_path).unwrap();

	Ok(())
}
