use anyhow::Result;
use clap::{App, Arg};
use libc::{c_char, uname, utsname};

// struct utsname {
//     char sysname[];    /* Operating system name (e.g., "Linux") */
//     char nodename[];   /* Name within "some implementation-defined network" */
//     char release[];    /* Operating system release (e.g., "2.6.28") */
//     char version[];    /* Operating system version */
//     char machine[];    /* Hardware identifier */
//  #ifdef _GNU_SOURCE
//     char domainname[]; /* NIS or YP domain name */
//  #endif
//  };

fn print_c_char(buf: &[c_char]) {
    let res = buf.iter().map(|&c| c as u8 as char).collect::<String>();
    print!("{}", res);
}

pub fn cli_command(arg: &[String]) -> Result<()> {
    let mut buf: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };

    let m = App::new("uname")
        .arg(Arg::with_name("all").short("a").long("all").help(
            "print all information, in the following order,\nexcept omit -p and -i if unknown:",
        ))
        .arg(
            Arg::with_name("kernel-name")
                .short("s")
                .long("kernel-name")
                .help("print the kernel name"),
        )
        .arg(
            Arg::with_name("nodename")
                .short("n")
                .long("nodename")
                .help("print the network node hostname"),
        )
        .arg(
            Arg::with_name("kernel-release")
                .short("r")
                .long("kernel-release")
                .help("print the kernel release"),
        )
        .arg(
            Arg::with_name("kernel-version")
                .short("v")
                .long("kernel-version")
                .help("print the kernel version"),
        )
        .arg(
            Arg::with_name("machine")
                .short("m")
                .long("machine")
                .help("print the machine hardware name"),
        )
        .get_matches_from(arg);

    // -p, --processor          print the processor type (non-portable)
    // -i, --hardware-platform  print the hardware platform (non-portable)
    // -o, --operating-system   print the operating system

    unsafe {
        uname(&mut buf as *mut libc::utsname);
    };

    if m.is_present("kernel-name")
        || m.is_present("all")
        || (!m.is_present("nodename")
            && !m.is_present("kernel-release")
            && !m.is_present("kernel-version")
            && !m.is_present("machine"))
    {
        print_c_char(&buf.sysname);
        print!(" ");
    }

    if m.is_present("nodename") || m.is_present("all") {
        print_c_char(&buf.nodename);
        print!(" ");
    }

    if m.is_present("kernel-release") || m.is_present("all") {
        print_c_char(&buf.release);
        print!(" ");
    }

    if m.is_present("kernel-version") || m.is_present("all") {
        print_c_char(&buf.version);
        print!(" ");
    }

    if m.is_present("machine") || m.is_present("all") {
        print_c_char(&buf.machine);
        print!(" ");
    }

    println!();

    Ok(())
}
