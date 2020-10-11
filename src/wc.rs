use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
#[clap(name = "wc")]
struct Opts {
    #[clap(short, long, about = "print the newline counts")]
    lines: Option<usize>,
    #[clap(short, long, about = "print the word counts")]
    words: Option<usize>,
    #[clap(short = 'c', long, about = "print the byte counts")]
    bytes: Option<usize>,
    #[clap(name = "FILE")]
    files: Vec<String>,
}

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
    // parse option
    let opts = Opts::parse_from(arg);

    let print_control = if opts.lines.is_none() && opts.words.is_none() && opts.bytes.is_none() {
        (true, true, true)
    } else {
        (
            opts.lines.is_some(),
            opts.words.is_some(),
            opts.bytes.is_some(),
        )
    };

    let files = if opts.files.is_empty() {
        vec!["-".into()]
    } else {
        opts.files
    };

    let mut result = Vec::<((usize, usize, usize), String)>::new();

    for f in files {
        let mut input = String::new();

        if f == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut input)?;
        } else {
            let file = File::open(&f)?;
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut input)?;
        }

        result.push((word_count(&input), f));
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
