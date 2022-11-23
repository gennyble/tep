use std::{fmt, str::FromStr};

use camino::Utf8PathBuf;
use getopts::Options;

pub struct CliArgs {
	pub palette_path: Option<Utf8PathBuf>,
	pub input_path: Utf8PathBuf,
	pub output_path: Utf8PathBuf,
}

impl CliArgs {
	fn usage(program: &str, opts: &Options) -> String {
		opts.usage(&format!("usage: {} INPUT OUTPUT [options]", program))
	}

	pub fn parse() -> Result<Option<Self>, Error> {
		let program = std::env::args().next().unwrap_or(String::from("tep"));
		let args: Vec<String> = std::env::args().skip(1).collect();

		let mut opts = Options::new();
		opts.optopt("p", "palette", "Use the palette at the provided path instead of the on in the image file. If there is a palette in the image file it's thrown out.", "PATH");
		opts.optflag("h", "help", "Print this message");
		let matches = opts.parse(&args)?;

		if matches.opt_present("h") {
			println!("{}", Self::usage(&program, &opts));
			return Ok(None);
		}

		let palette_path = match matches.opt_str("p") {
			None => None,
			Some(p) => {
				let path = Utf8PathBuf::from(p);
				if !path.exists() {
					return Err(Error::PaletteInvalid { path });
				} else {
					Some(path)
				}
			}
		};

		let input_path = match matches.free.get(0) {
			None => return Err(Error::NoInput),
			Some(p) => {
				let path = Utf8PathBuf::from(p);
				if !path.exists() {
					return Err(Error::InputInvalid { path });
				} else {
					path
				}
			}
		};

		let output_path = match matches.free.get(1) {
			None => return Err(Error::NoOutput),
			Some(p) => Utf8PathBuf::from(p),
		};

		Ok(Some(Self {
			palette_path,
			input_path,
			output_path,
		}))
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("{0}")]
	Getopts(#[from] getopts::Fail),
	#[error("the palette at '{path}' does not exist")]
	PaletteInvalid { path: Utf8PathBuf },
	#[error("no Input file given")]
	NoInput,
	#[error("the input file '{path}' does not exist")]
	InputInvalid { path: Utf8PathBuf },
	#[error("no output file given")]
	NoOutput,
}
