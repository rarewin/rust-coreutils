extern crate base64;
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Base64Error(String);

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}\n", self.0)
    }
}

impl error::Error for Base64Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn base64<R: BufRead>(
    f: &mut dyn std::io::Write,
    r: &mut R,
    m: &clap::ArgMatches<'_>,
) -> Result<(), Base64Error> {
    let wrap = match m.value_of("wrap") {
        Some(d) => d,
        None => {
            return Err(Base64Error("invalid parameter for --wrap".into()));
        }
    };

    let wrap = match wrap.parse::<usize>() {
        Ok(d) => d,
        Err(e) => {
            return Err(Base64Error(format!("{}", e)));
        }
    };

    let buf = match r.fill_buf() {
        Ok(d) => d,
        Err(e) => {
            return Err(Base64Error(format!("{}", e)));
        }
    };

    if buf.len() == 0 {
        return Ok(());
    }

    let result = base64::encode(buf);
    let len = result.len();

    for i in 0..((len - 1) / wrap) {
        writeln!(f, "{}", &result[(i * wrap)..((i + 1) * wrap)]).unwrap();
    }
    writeln!(f, "{}", &result[(len - (len % wrap))..]).unwrap();

    Ok(())
}

pub fn cli_command(arg: &[String]) -> Result<(), Box<dyn error::Error>> {
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
        let file = File::open(filename)?;
        let mut file = BufReader::new(file);

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        match base64(&mut stdout, &mut file, &m) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
    } else {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdin = stdin.lock();
        let mut stdout = stdout.lock();

        match base64(&mut stdout, &mut stdin, &m) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
    };

    Ok(())
}
