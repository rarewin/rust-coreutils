use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, Result};
use clap::{App, Arg};

fn cat_data<R: BufRead>(r: &mut R, m: &clap::ArgMatches, line_start: u32) -> u32 {
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

    line
}

pub fn cli_command(arg: &[String]) -> Result<()> {
    // parse option
    let m = App::new("cat")
        .arg(Arg::new("FILE").multiple(true))
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .about("number all output lines"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .about("number nonempty output lines, overrides -n"),
        )
        .arg(
            Arg::new("show-tabs")
                .short('T')
                .long("show-tabs")
                .about("display TAB characters as ^I"),
        )
        .arg(
            Arg::new("show-ends")
                .short('E')
                .long("show-ends")
                .about("display $ at end of each line"),
        )
        .arg(
            Arg::new("squeeze-blank")
                .short('s')
                .long("squeeze-blank")
                .about("suppress repeated empty output lines"),
        )
        .arg(Arg::new("u").short('u').about("(ignored)"))
        .get_matches_from(arg);

    let args: Vec<_> = if !m.is_present("FILE") {
        vec!["-"]
    } else if let Some(files) = m.values_of("FILE") {
        files.collect()
    } else {
        return Err(anyhow!("invalid argument for FILE"));
    };

    let mut line = 1;
    for arg in args {
        if arg == "-" {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            line = cat_data(&mut stdin, &m, line);
        } else {
            let mut file = BufReader::new(File::open(arg)?);
            line = cat_data(&mut file, &m, line);
        };
    }

    Ok(())
}
