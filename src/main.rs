mod cli;
mod tep;

use cli::CliArgs;
use tep::{Palette, Tep};

fn main() -> eyre::Result<()> {
	let cli = match CliArgs::parse()? {
		None => return Ok(()),
		Some(c) => c,
	};

	match cli.palette_path {
		None => {
			let tep = Tep::file(cli.input_path).unwrap();
			tep.save_as_png(cli.output_path).unwrap();
		}
		Some(path) => {
			let palette = Palette::file(path).unwrap();
			let tep = Tep::with_palette(palette, cli.input_path).unwrap();
			tep.save_as_png(cli.output_path).unwrap()
		}
	}

	Ok(())
}
