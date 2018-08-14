extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn cat_data<R: BufRead>(r: &mut R, m: &clap::ArgMatches<'_>, line_start: u32) -> u32 {
    let mut input = String::new();
    let mut line: u32 = line_start;

    loop {
        match r.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if m.is_present("number-nonblank") {
                    print!("        ");
                    line -= 1;
                } else if m.is_present("number") || m.is_present("number-nonblank") {
                    print!("{:>6}  ", line);
                }
                print!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        input.clear();
        line += 1;
    }
    return line;
}

fn main() {
    // parse option
    let m = App::new("cat")
        .arg(Arg::with_name("FILE").multiple(true))
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines"),
        ).arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n"),
        ).get_matches();

    let args: Vec<_> = if !m.is_present("FILE") {
        vec!["-"]
    } else {
        m.values_of("FILE").unwrap().collect()
    };

    let mut line = 1;
    for i in 0..args.len() {
        if args[i] == "-" {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            line = cat_data(&mut stdin, &m, line);
        } else {
            let mut file = BufReader::new(File::open(args[i]).unwrap());
            line = cat_data(&mut file, &m, line);
        };
    }
}
