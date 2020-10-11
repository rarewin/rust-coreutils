use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::Result;
use clap::Clap;

#[derive(Clap)]
#[clap(name = "cat")]
pub struct Opts {
    #[clap(short, long, about = "number all output lines")]
    number: bool,

    #[clap(
        short = 'b',
        long = "number-nonblank",
        about = "number nonempty output lines, overrides -n"
    )]
    number_nonblank: bool,

    #[clap(
        short = 'T',
        long = "show-tabs",
        about = "display TAB characters as ^I"
    )]
    show_tabs: bool,

    #[clap(
        short = 'E',
        long = "show-ends",
        about = "display $ at end of each line"
    )]
    show_ends: bool,

    #[clap(
        short = 's',
        long = "squeeze-blank",
        about = "suppress repeated empty output lines"
    )]
    squeeze_blank: bool,

    #[clap(short = 'u', about = "(ignored)")]
    _u: bool,

    #[clap(name = "FILE")]
    files: Vec<String>,
}

fn cat_data<R: BufRead>(r: &mut R, opts: &Opts, line_start: u32) -> u32 {
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
                if opts.squeeze_blank {
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
                if opts.number_nonblank {
                    print!("        ");
                    line -= 1;
                } else if opts.number || opts.number_nonblank {
                    print!("{:>6}  ", line);
                }

                // convert TAB to "^I"
                if opts.show_tabs {
                    input = input.replace("\t", "^I");
                }

                // output "$" at the end of the lines
                if opts.show_ends {
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
    let opts = Opts::parse_from(arg);

    let args: Vec<_> = if opts.files.is_empty() {
        vec!["-".into()]
    } else {
        opts.files.clone()
    };

    let mut line = 1;
    for arg in args {
        if arg == "-" {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            line = cat_data(&mut stdin, &opts, line);
        } else {
            let mut file = BufReader::new(File::open(arg)?);
            line = cat_data(&mut file, &opts, line);
        };
    }

    Ok(())
}
