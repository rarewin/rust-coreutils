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
        .arg(Arg::new("all").short('a').long("all").about(
            "print all information, in the following order,\nexcept omit -p and -i if unknown:",
        ))
        .arg(
            Arg::new("kernel-name")
                .short('s')
                .long("kernel-name")
                .about("print the kernel name"),
        )
        .arg(
            Arg::new("nodename")
                .short('n')
                .long("nodename")
                .about("print the network node hostname"),
        )
        .arg(
            Arg::new("kernel-release")
                .short('r')
                .long("kernel-release")
                .about("print the kernel release"),
        )
        .arg(
            Arg::new("kernel-version")
                .short('v')
                .long("kernel-version")
                .about("print the kernel version"),
        )
        .arg(
            Arg::new("machine")
                .short('m')
                .long("machine")
                .about("print the machine hardware name"),
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
