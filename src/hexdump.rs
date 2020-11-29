use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use clap::Clap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HexdumpError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Clap, Debug)]
#[clap(name = "hexdump")]
struct Opts {
    files: Vec<PathBuf>,
}

pub fn hexdump<R: Read>(r: &mut R) -> Result<(), HexdumpError> {
    for byte in r.bytes() {
        println!("{:02x}", byte?);
    }

    Ok(())
}

pub fn cli_command(arg: &[String]) -> Result<(), HexdumpError> {
    let opts = Opts::parse_from(arg);

    let mut file = BufReader::new(File::open(&opts.files[0])?);

    hexdump(&mut file)?;

    Ok(())
}
