use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use anyhow::Result;
use clap::{App, Arg};

/// Count the numbers of returns, words, and bytes.
///
/// # Arguments
///
/// * `buf` - input string
///
/// # Example
///
/// ```
/// use rust_coreutils::wc;
///
/// assert_eq!(wc::word_count("hoge fuga moge"), (0, 3, 14));
/// assert_eq!(wc::word_count("hoge fuga moge\n"), (1, 3, 15));
/// ```
pub fn word_count(input: &str) -> (usize, usize, usize) {
    let nlines = input.matches('\n').count();
    let nwords = input.split_whitespace().count();
    (nlines, nwords, input.len())
}

#[test]
fn test_word_count() {
    struct Test<'a> {
        input: &'a str,
        expect: (usize, usize, usize),
    };

    let tests = vec![
        Test {
            input: "hoge fuga moge",
            expect: (0, 3, 14),
        },
        Test {
            input: "hoge fuga moge\n",
            expect: (1, 3, 15),
        },
        Test {
            input: "hoge\nge\nfuge\nmoge\n\nmoge",
            expect: (5, 5, 23),
        },
    ];

    for t in tests {
        assert_eq!(word_count(t.input), t.expect);
    }
}

pub fn cli_command(arg: &[String]) -> Result<()> {
    let m = App::new("wc")
        .arg(Arg::with_name("FILE").multiple(true))
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("print the newline counts"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("print the word counts"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the byte counts"),
        )
        .get_matches_from(arg);

    let print_control =
        if !m.is_present("lines") && !m.is_present("words") && !m.is_present("bytes") {
            (true, true, true)
        } else {
            (
                m.is_present("lines"),
                m.is_present("words"),
                m.is_present("bytes"),
            )
        };

    let files = if let Some(it) = m.values_of("FILE") {
        it.collect()
    } else {
        vec!["-"]
    };

    let mut result = Vec::<((usize, usize, usize), String)>::new();

    for f in files {
        let mut input = String::new();

        if f == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut input)?;
        } else {
            let file = File::open(f)?;
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut input)?;
        }

        result.push((word_count(&input), f.into()));
    }

    print_result(&result, print_control);

    Ok(())
}

fn print_result(results: &[((usize, usize, usize), String)], control: (bool, bool, bool)) {
    for r in results {
        if control.0 {
            print!("{}", (r.0).0);
        }

        if control.1 {
            print!(" {}", (r.0).1);
        }

        if control.2 {
            print!(" {}", (r.0).2);
        }

        println!(" {}", r.1);
    }
}

/*
fn main() {
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
*/
