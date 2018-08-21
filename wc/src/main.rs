extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs, io};

fn wc_data<R: BufRead>(r: &mut R) -> (usize, usize, usize) {
    let mut input = String::new();
    let mut nlines: usize = 0;
    let mut nwords: usize = 0;
    let mut nbytes: usize = 0;

    loop {
        match r.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                nbytes += n;
                nwords += input.split_whitespace().count();
                nlines += 1;

                input.clear();
            }
            Err(e) => println!("{}", e),
        }
    }

    (nlines, nwords, nbytes)
}

fn digits(number: usize) -> usize {
    let mut d = 0;
    let mut n = number;

    while n != 0 {
        d += 1;
        n /= 10;
    }

    return d;
}

fn main() {
    let m = App::new("wc")
        .arg(Arg::with_name("FILE").multiple(true))
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("print the newline counts"),
        ).arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("print the word counts"),
        ).arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the byte counts"),
        ).get_matches();
    let mut results: Vec<(usize, usize, usize, &str)> = Vec::new();
    let mut total: (usize, usize, usize) = (0, 0, 0);

    let mut print_lines = m.is_present("lines");
    let mut print_words = m.is_present("words");
    let mut print_bytes = m.is_present("bytes");

    if !print_lines && !print_words && !print_bytes {
        print_lines = true;
        print_words = true;
        print_bytes = true;
    };

    let args: Vec<_> = if !m.is_present("FILE") {
        vec![""]
    } else {
        m.values_of("FILE").unwrap().collect()
    };

    let mut is_special_file = false;

    for a in args {
        if a == "-" || a == "" {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let (nl, nw, nb) = wc_data(&mut stdin);
            results.push((nl, nw, nb, a));
            total = (total.0 + nl, total.1 + nw, total.2 + nb);
            is_special_file = true;
        } else {
            let mut file = BufReader::new(File::open(a).unwrap());
            let (nl, nw, nb) = wc_data(&mut file);
            results.push((nl, nw, nb, a));
            total = (total.0 + nl, total.1 + nw, total.2 + nb);

            if !is_special_file && !fs::metadata(a).unwrap().is_file() {
                is_special_file = true;
            }
        }
    }

    let l = results.len();
    let mut width: usize = 0;

    if print_lines {
        width = digits(total.0);
    }

    if print_words {
        width = digits(total.1);
    }

    if print_bytes {
        width = digits(total.2);
    }

    if is_special_file && width < 7 {
        width = 7
    }

    for r in results {
        if print_lines {
            print!("{:>width$} ", r.0, width = width);
        }
        if print_words {
            print!("{:>width$} ", r.1, width = width);
        }
        if print_bytes {
            print!("{:>width$} ", r.2, width = width);
        }

        println!("{}", r.3);
    }

    if l > 1 {
        println!(
            "{:>width$} {:>width$} {:>width$} total",
            total.0,
            total.1,
            total.2,
            width = width
        );
    }
}
