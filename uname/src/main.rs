extern crate clap;
extern crate libc;

use clap::{App, Arg};
// use libc::{c_int, c_char, c_void, size_t};
use libc::{c_char, utsname, uname};

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

fn main() {

    let mut buf: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };

    let m = App::new("uname")
        .arg(Arg::with_name("kernel-name")
                 .short("s")
                 .long("kernel-name")
                 .help("print the kernel name"))
        .arg(Arg::with_name("nodename")
                 .short("n")
                 .long("nodename")
                 .help("print the network node hostname"))
        .get_matches();

    unsafe {
        uname(&mut buf as *mut libc::utsname);
    };

    // let cstring = CString::new(buf.sysname.to_vec()).unwrap();

    // let s = String::from_iter(buf.sysname);
    // buf.sysname.iter().map(|c| *c).collect::<String>();
    //let res = buf.sysname
    //    .iter()
    //    .map(|&c| c as u8 as char)
    //    .collect::<String>();

    if m.is_present("kernel-name") {
        print_c_char(&buf.sysname);
        print!(" ");
    }

    if m.is_present("nodename") {
        print_c_char(&buf.nodename);
        print!(" ");
    }
    // print_c_char(&buf.release);
    // print!(" ");
    // print_c_char(&buf.version);
    // print!(" ");
    // print_c_char(&buf.machine);
    // print!(" ");
    // print_c_char(&buf.domainname);

    println!("");
}
