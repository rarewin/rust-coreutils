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
        .get_matches();
    let mut results: Vec<(usize, usize, usize, &str)> = Vec::new();
    let mut total: (usize, usize, usize) = (0, 0, 0);

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
        };
    }

    let l = results.len();
    let width = digits(total.2);

    let width = if is_special_file && width < 7 {
        7
    } else {
        width
    };

    for r in results {
        println!(
            "{:>width$} {:>width$} {:>width$} {}",
            r.0,
            r.1,
            r.2,
            r.3,
            width = width
        );
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
