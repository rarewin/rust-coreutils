extern crate base64;
extern crate clap;

use std::fs::File;
use std::io::{self, Read, Write};

use anyhow::{Error, Result};
use clap::{App, Arg};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Base64Error {
    #[error("invalid parameter for {0}")]
    InvalidParam(String),
}

fn base64<R: Read, W: Write>(f: &mut W, r: &mut R, m: &clap::ArgMatches<'_>) -> Result<()> {
    let wrap = m
        .value_of("wrap")
        .ok_or_else(|| Base64Error::InvalidParam("--wrap".to_string()))?
        .parse::<usize>()
        .or_else(|_| Err(Error::new(Base64Error::InvalidParam("--wrap".to_string()))))?;
    let mut buf = Vec::new();

    r.read_to_end(&mut buf)?;

    let result = base64::encode(&buf);
    let len = result.len();

    for i in 0..(len / wrap) {
        writeln!(f, "{}", &result[(i * wrap)..((i + 1) * wrap)])?;
    }

    writeln!(f, "{}", &result[(len / wrap) * wrap..])?;

    Ok(())
}

pub fn cli_command(arg: &[String]) -> Result<()> {
    let m = App::new("base64")
        .about("Base64 encode or decode FILE, or standard input, to standard output.
With no FILE, or when FILE is -, read standard input.")
        .arg(Arg::with_name("FILE"))
        .arg(Arg::with_name("wrap")
             .short("w")
             .long("wrap")
             .takes_value(true)
             .value_name("COLS")
             .number_of_values(1)
             .help("wrap encoded lines after COLS character (default 76).\nUse 0 to disable line wrapping")
             .default_value("76"),
        ).get_matches_from(arg);

    if m.is_present("FILE") {
        let filename = m.value_of("FILE").unwrap();
        let mut file = File::open(filename)?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        base64(&mut stdout, &mut file, &m)?;
    } else {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdin = stdin.lock();
        let mut stdout = stdout.lock();

        base64(&mut stdout, &mut stdin, &m)?;
    }

    Ok(())
}
