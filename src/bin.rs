use std::fs::File;
use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

use qrust::create_qr_code;
use qrust::qr::error_correction::ErrorCorrectionLevel;
use qrust::qr::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "qrgen", about = "Generate a QR code")]
struct Opts {
    #[structopt(
        short = "i",
        parse(from_os_str),
        help = "Input file to read data from; if not set, reads from stdin"
    )]
    input: Option<PathBuf>,

    #[structopt(
        short = "e",
        long = "ecl",
        help = "Error correction level (low, medium, quartile or high; default medium)"
    )]
    ecl: Option<ErrorCorrectionLevel>,
}

fn run(opts: Opts) -> Result<(), Error> {
    let mut input_stream: Box<dyn io::Read> = match opts.input {
        None => Box::new(io::stdin()),
        Some(i) => Box::new(File::open(i).map_err(|e| e.to_string())?),
    };
    let mut data = String::new();
    input_stream
        .read_to_string(&mut data)
        .map_err(|e| e.to_string())?;
    let input = data.trim();
    let ecl = opts.ecl.unwrap_or(ErrorCorrectionLevel::Medium);
    create_qr_code(input, ecl)
}

pub fn main() {
    if let Err(e) = run(Opts::from_args()) {
        println!("Error: {:#?}", e)
    }
}
