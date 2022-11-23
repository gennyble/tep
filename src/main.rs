mod cli;
mod tep;

use cli::CliArgs;
use tep::{Palette, Tep};

fn main() {
	let cli = match CliArgs::parse() {
		Ok(Some(c)) => c,
		Ok(None) => return,
		Err(e) => {
			eprintln!("{}", e);
			std::process::exit(-1);
		}
	};

	let tep = match cli.palette_path {
		None => Tep::file(cli.input_path).unwrap(),
		Some(path) => {
			let palette = Palette::file(path).unwrap();
			Tep::with_palette(palette, cli.input_path).unwrap()
		}
	};

	tep.save_as_png(cli.output_path).unwrap();
}
