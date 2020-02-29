use std::error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

fn cat_data<R: BufRead>(r: &mut R, m: &clap::ArgMatches<'_>, line_start: u32) -> u32 {
    let mut input = String::new();
    let mut line: u32 = line_start;
    let mut consecutive_blank_line: bool = false;

    loop {
        match r.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                // manage "-s" option
                if m.is_present("squeeze-blank") {
                    if n == 1 {
                        if consecutive_blank_line {
                            input.clear();
                            continue;
                        }
                        consecutive_blank_line = true;
                    } else {
                        consecutive_blank_line = false;
                    }
                }

                // print line number
                if m.is_present("number-nonblank") {
                    print!("        ");
                    line -= 1;
                } else if m.is_present("number") || m.is_present("number-nonblank") {
                    print!("{:>6}  ", line);
                }

                // convert TAB to "^I"
                if m.is_present("show-tabs") {
                    input = input.replace("\t", "^I");
                }

                // output "$" at the end of the lines
                if m.is_present("show-ends") {
                    input = input.replace("\n", "$\n");
                }

                // print line
                print!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        input.clear();
        line += 1;
    }
    return line;
}

pub fn cli_command(arg: &[String]) -> Result<(), Box<dyn error::Error>> {
    // parse option
    let m = App::new("cat")
        .arg(Arg::with_name("FILE").multiple(true))
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n"),
        )
        .arg(
            Arg::with_name("show-tabs")
                .short("T")
                .long("show-tabs")
                .help("display TAB characters as ^I"),
        )
        .arg(
            Arg::with_name("show-ends")
                .short("E")
                .long("show-ends")
                .help("display $ at end of each line"),
        )
        .arg(
            Arg::with_name("squeeze-blank")
                .short("s")
                .long("squeeze-blank")
                .help("suppress repeated empty output lines"),
        )
        .arg(Arg::with_name("u").short("u").help("(ignored)"))
        .get_matches_from(arg);

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

    Ok(())
}
