use std::{collections::HashMap, fs::File, io::BufWriter, str::FromStr};

use camino::Utf8Path;
use png::{Encoder, Writer};

pub struct Tep {
	colours: Vec<ColourDefinition>,

	width: usize,
	height: usize,
	data: Vec<char>,
}

impl Tep {
	pub fn file<P: AsRef<Utf8Path>>(path: P) -> Result<Self, Error> {
		let string = std::fs::read_to_string(path.as_ref()).map_err(|e| Error::FileRead {
			path: path.as_ref().to_string(),
			error: Box::new(e),
		})?;
		let mut lines = string.lines().enumerate();

		let mut colours_string = String::new();
		let mut colours = vec![];
		loop {
			match lines.next() {
				None => {
					return Err(Error::NoImage);
				}
				Some((_ln, line)) if line.is_empty() => {
					break;
				}
				Some((_ln, line)) => {
					let def = Self::parse_colour_definition(line)?;
					colours_string.push(def.ident);
					colours.push(def);
				}
			}
		}

		let mut data = vec![];
		let mut width = None;
		let mut height = 0;
		loop {
			match lines.next() {
				None | Some((_, "")) => break,
				Some((ln, line)) => {
					let line_width = line.len();

					match width {
						None => width = Some(line_width),
						Some(width) if width != line_width => {
							return Err(Error::DifferingWidths {
								expected: width,
								ln,
								actual: line_width,
							});
						}
						_ => (),
					}

					for (idx, ch) in line.chars().enumerate() {
						if colours_string.contains(ch) {
							data.push(ch);
						} else {
							return Err(Error::UnknownIdentifier {
								ident: ch,
								line_idx: idx,
								ln,
							});
						}
					}

					height += 1;
				}
			}
		}

		if height == 0 {
			return Err(Error::NoImage);
		} else {
			Ok(Self {
				colours,
				width: width.unwrap(),
				height,
				data,
			})
		}
	}

	pub fn save_as_png<P: AsRef<Utf8Path>>(&self, path: P) -> Result<(), Error> {
		let mut colours = self.colours.clone();

		// We're sorting by the alpha channel so that all of the transparent values come before the perfectly
		// opaque ones. This is neccesary for creating the PNG tRNS block for paletted images
		colours.sort_by(|a, b| a.colour.a.cmp(&b.colour.b));

		if colours.len() <= 256 {
			let trns = Self::trns(&colours);
			let palette = Self::palette(&colours);
			let data = Self::paletted_date(colours, &self.data);

			let file = File::create(path.as_ref()).map_err(|e| Error::FileWrite {
				path: path.as_ref().to_string(),
				error: Box::new(e),
			})?;
			let bufw = BufWriter::new(file);

			let mut encoder = Encoder::new(bufw, self.width as u32, self.height as u32);
			encoder.set_color(png::ColorType::Indexed);
			encoder.set_depth(png::BitDepth::Eight);

			if !trns.is_empty() {
				encoder.set_trns(trns);
			}

			encoder.set_palette(palette);

			let mut writer = encoder
				.write_header()
				.map_err(|e| Error::PngEncodingError { error: e })?;
			writer
				.write_image_data(&data)
				.map_err(|e| Error::PngEncodingError { error: e })
		} else {
			todo!("Cannot currently handle images with higher than 256 colours!")
		}
	}

	fn paletted_date(colours: Vec<ColourDefinition>, data: &[char]) -> Vec<u8> {
		let mut ret = vec![];

		for idx in data {
			let colour_index = colours
				.iter()
				.enumerate()
				.find(|(_, def)| def.ident == *idx)
				.unwrap()
				.0;

			ret.push(colour_index as u8);
		}

		ret
	}

	fn palette(colours: &[ColourDefinition]) -> Vec<u8> {
		let mut ret = vec![];

		for colour in colours {
			ret.push(colour.colour.r);
			ret.push(colour.colour.g);
			ret.push(colour.colour.b);
		}

		ret
	}

	/// Expects the vec to be sorted by alpha channel
	fn trns(colours: &Vec<ColourDefinition>) -> Vec<u8> {
		let mut alphas = vec![];

		for definition in colours {
			if definition.colour.is_opaque() {
				break;
			}

			alphas.push(definition.colour.a);
		}

		alphas
	}

	fn parse_colour_definition(s: &str) -> Result<ColourDefinition, Error> {
		let trimmed = s.trim();

		match trimmed.split_once(':') {
			None => Err(Error::MalformedColourDefinition {
				raw: trimmed.to_string(),
			}),
			Some((ident, colour)) => {
				let ident = ident.trim();
				let colour = colour.trim();

				if ident.len() != 1 {
					Err(Error::MalformedColourDefinition {
						raw: trimmed.to_string(),
					})
				} else {
					let ident = ident.chars().next().unwrap();
					let colour = colour
						.parse()
						.map_err(|e| Error::malformed_colour(ident, e))?;

					Ok(ColourDefinition { ident, colour })
				}
			}
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Could not read the file '{path}' because: {error}")]
	FileRead {
		path: String,
		error: Box<dyn std::error::Error>,
	},
	#[error("Could not write to the file at '{path}' because: {error}")]
	FileWrite {
		path: String,
		error: Box<dyn std::error::Error>,
	},
	#[error("Error encoding PNG: {error}")]
	PngEncodingError { error: png::EncodingError },
	#[error("'{raw}' is not a valid colour definition")]
	MalformedColourDefinition { raw: String },
	#[error("The colour for '{ident}' is invalid: '{error}'")]
	MalformedColour {
		ident: char,
		error: ColourParseError,
	},
	#[error("There was no image following the colours!")]
	NoImage,
	#[error("The image lines differ in width. The first line was {expected} but the line {ln} was {actual}")]
	DifferingWidths {
		expected: usize,
		ln: usize,
		actual: usize,
	},
	#[error("The identifier '{ident}' found at {line_idx} on line {ln} was not defined")]
	UnknownIdentifier {
		ident: char,
		line_idx: usize,
		ln: usize,
	},
}

impl Error {
	fn malformed_colour(ident: char, error: ColourParseError) -> Self {
		Self::MalformedColour { ident, error }
	}
}

#[derive(Copy, Clone, Debug)]
pub struct ColourDefinition {
	ident: char,
	colour: Colour,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

impl Colour {
	pub fn grey(g: u8) -> Self {
		Self::rgb(g, g, g)
	}

	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b, a: 255 }
	}

	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	/// true if the colour is not transparent at all; if the alpha channel is 255
	pub fn is_opaque(&self) -> bool {
		self.a == 255
	}

	fn parse_hex(s: &str) -> Result<Self, <Self as FromStr>::Err> {
		let mut chars = s.chars();

		macro_rules! hexpair {
			($low:expr, $high:expr) => {
				Self::parse_hexpair($low, $high)
					.map_err(|_| ColourParseError::InvalidHex { raw: s.to_string() })
			};
		}

		match s.len() {
			1 => {
				// Short grey
				let grey = chars.next().unwrap();
				Ok(Self::grey(hexpair!(grey, grey)?))
			}
			2 => {
				// Long grey
				let low = chars.next().unwrap();
				let high = chars.next().unwrap();
				Ok(Self::grey(hexpair!(low, high)?))
			}
			3 => {
				// Short full colour
				let rch = chars.next().unwrap();
				let gch = chars.next().unwrap();
				let bch = chars.next().unwrap();

				let r = hexpair!(rch, rch)?;
				let g = hexpair!(gch, gch)?;
				let b = hexpair!(bch, bch)?;

				Ok(Self::rgb(r, g, b))
			}
			4 => {
				// Short RGBA
				let rch = chars.next().unwrap();
				let gch = chars.next().unwrap();
				let bch = chars.next().unwrap();
				let ach = chars.next().unwrap();

				let r = hexpair!(rch, rch)?;
				let g = hexpair!(gch, gch)?;
				let b = hexpair!(bch, bch)?;
				let a = hexpair!(ach, ach)?;

				Ok(Self::rgba(r, g, b, a))
			}
			6 => {
				// Long full colour
				let rlow = chars.next().unwrap();
				let rhigh = chars.next().unwrap();
				let glow = chars.next().unwrap();
				let ghigh = chars.next().unwrap();
				let blow = chars.next().unwrap();
				let bhigh = chars.next().unwrap();

				let r = hexpair!(rlow, rhigh)?;
				let g = hexpair!(glow, ghigh)?;
				let b = hexpair!(blow, bhigh)?;

				Ok(Self::rgb(r, g, b))
			}
			8 => {
				// Long full colour
				let rlow = chars.next().unwrap();
				let rhigh = chars.next().unwrap();
				let glow = chars.next().unwrap();
				let ghigh = chars.next().unwrap();
				let blow = chars.next().unwrap();
				let bhigh = chars.next().unwrap();
				let alow = chars.next().unwrap();
				let ahigh = chars.next().unwrap();

				let r = hexpair!(rlow, rhigh)?;
				let g = hexpair!(glow, ghigh)?;
				let b = hexpair!(blow, bhigh)?;
				let a = hexpair!(alow, ahigh)?;

				Ok(Self::rgba(r, g, b, a))
			}
			_ => Err(ColourParseError::InvalidHex { raw: s.to_string() }),
		}
	}

	fn parse_hexpair(low: char, high: char) -> Result<u8, <Self as FromStr>::Err> {
		let pair = format!("{low}{high}");
		u8::from_str_radix(&pair, 16).map_err(|_| ColourParseError::InvalidHex { raw: pair })
	}
}

impl FromStr for Colour {
	type Err = ColourParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().next() {
			None => Err(ColourParseError::Empty),
			Some('#') => Self::parse_hex(&s[1..]),
			_ => todo!(),
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum ColourParseError {
	#[error("Colour was empty!")]
	Empty,
	#[error("{raw} is not a valid hex colour code")]
	InvalidHex { raw: String },
}

#[cfg(test)]
mod test {
	use super::Colour;

	fn assert_colour_parsed(raw: &str, expected: Colour) {
		assert_eq!(raw.parse::<Colour>().unwrap(), expected)
	}

	#[test]
	fn parse_all_hex() {
		let g = "#1";
		let lg = "#1a";
		let short = "#123";
		let shorta = "#1234";
		let long = "#1a2b3c";
		let longa = "#1a2b3c4d";

		assert_colour_parsed(g, Colour::grey(0x11));
		assert_colour_parsed(lg, Colour::grey(0x1a));
		assert_colour_parsed(short, Colour::rgb(0x11, 0x22, 0x33));
		assert_colour_parsed(shorta, Colour::rgba(0x11, 0x22, 0x33, 0x44));
		assert_colour_parsed(long, Colour::rgb(0x1a, 0x2b, 0x3c));
		assert_colour_parsed(longa, Colour::rgba(0x1a, 0x2b, 0x3c, 0x4d));
	}
}
