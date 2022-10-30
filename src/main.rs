mod tep;

use tep::Tep;

fn main() -> eyre::Result<()> {
	let ifile = std::env::args().nth(1).unwrap();
	let ofile = std::env::args().nth(2).unwrap();

	let tep = Tep::file(ifile).unwrap();
	tep.save_as_png(ofile).unwrap();

	Ok(())
}
