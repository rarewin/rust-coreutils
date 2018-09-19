extern crate clap;

use clap::{App, Arg};

fn main() {
    // parse option
    let m = App::new("basename")
        .about(
            "Print NAME with any leading directory components removed.
If specified, also remove a trailing SUFFIX.",
        ).usage("basename NAME [SUFFIX]  or  basename [FLAGS]  NAME...")
        .arg(Arg::with_name("NAME").required(true).multiple(true))
        .arg(
            Arg::with_name("multiple")
                .short("a")
                .long("multiple")
                .help("support multiple arguments and treat each as a NAME"),
        ).arg(
            Arg::with_name("suffix")
                .short("s")
                .long("suffix")
                .value_name("SUFFIX")
                .takes_value(true)
                .number_of_values(1)
                .help("remove a trailing SUFFIX; implies -a"),
        ).get_matches();

    let names: Vec<_> = m.values_of("NAME").unwrap().collect();

    if m.is_present("multiple") {
        for s in names {
            let d: Vec<&str> = s.split('/').collect();
            println!("{}", d[d.len() - 1]);
        }
    }
}
