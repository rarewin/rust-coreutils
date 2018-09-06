#![feature(bufreader_buffer)]

extern crate base64;
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn base64<R: BufRead>(r: &mut R, m: &clap::ArgMatches<'_>) {
    let wrap = if m.is_present("wrap") {
        m.value_of("wrap").unwrap().parse().unwrap()
    } else {
        76 // default value
    };

    let buf = r.fill_buf().unwrap();

    if buf.len() > 0 {
        let result = base64::encode(buf);
        let mut i = 0;
        while i < ((result.len() - 1) / wrap) {
            println!("{}", &result[(i * wrap)..((i + 1) * wrap)]);
            i += 1;
        }
        println!("{}", &result[(i * wrap)..]);
    }
}

fn main() {
    let m = App::new("base64")
        .arg(Arg::with_name("FILE"))
        .arg(Arg::with_name("wrap")
             .short("w")
             .long("wrap")
             .takes_value(true)
             .value_name("COLS")
             .number_of_values(1)
             .help("wrap encoded lines after COLS character (default 76).\nUse 0 to disable line wrapping"),
        ).get_matches();

    if m.is_present("FILE") {
        let mut file = BufReader::new(File::open(m.value_of("FILE").unwrap()).unwrap());
        base64(&mut file, &m);
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        base64(&mut stdin, &m);
    };
}
