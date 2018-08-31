#![feature(bufreader_buffer)]

extern crate base64;
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let args: Vec<_> = if !m.is_present("FILE") {
        vec!["-"]
    } else {
        m.values_of("FILE").unwrap().collect()
    };

    // let e = base64::encode(b"Hello World");
    // println!("{}", e);

    let mut file = BufReader::new(File::open(args[0]).unwrap());

    let wrap = if m.is_present("wrap") {
        m.value_of("wrap").unwrap().parse().unwrap()
    } else {
        76 // default value
    };

    println!("{}", wrap);

    if file.fill_buf().unwrap().len() > 0 {
        let result = base64::encode(file.buffer());
        println!("{}", result);
    }
}
