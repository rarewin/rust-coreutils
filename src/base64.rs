use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
#[clap(
    name = "base64",
    about = "Base64 encode or decode FILE, or standard input, to standard output.
With no FILE, or when FILE is -, read standard input."
)]
struct Opts {
    #[clap(
        short,
        long,
        name = "COLS",
        about = "wrap encoded lines after COLS character (default 76).
Use 0 to disable line wrapping"
    )]
    wrap: Option<usize>,

    #[clap(name = "FILE")]
    file_name: Option<PathBuf>,
}

fn base64<R: Read, W: Write>(f: &mut W, r: &mut R, opts: &Opts) -> Result<()> {
    let wrap = if let Some(w) = opts.wrap { w } else { 76 };
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
    let opts = Opts::parse_from(arg);

    if let Some(filename) = &opts.file_name {
        let mut file = File::open(filename)?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        base64(&mut stdout, &mut file, &opts)?;
    } else {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdin = stdin.lock();
        let mut stdout = stdout.lock();

        base64(&mut stdout, &mut stdin, &opts)?;
    }

    Ok(())
}
