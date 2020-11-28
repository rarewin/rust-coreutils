use clap::Clap;
use libc::{c_char, uname, utsname};
use thiserror::Error;

#[derive(Clap)]
#[clap(
    name = "uname",
    about = "Print certain system information.  With no OPTION, same as -s."
)]
pub struct Opts {
    #[clap(
        short,
        long,
        about = "print all information, in the following order,
except omit -p and -i if unknown:"
    )]
    all: bool,

    #[clap(short = 's', long = "kernel-name", about = "print the kernel name")]
    kernel_name: bool,

    #[clap(short, long, about = "print the network node hostname")]
    nodename: bool,

    #[clap(
        short = 'r',
        long = "kernel-release",
        about = "print the kernel release"
    )]
    kernel_release: bool,

    #[clap(
        short = 'v',
        long = "kernel-version",
        about = "print the kernel version"
    )]
    kernel_version: bool,

    #[clap(short, long, about = "print the machine hardware name")]
    machine: bool,
    // -p, --processor          print the processor type (non-portable)
    // -i, --hardware-platform  print the hardware platform (non-portable)
    // -o, --operating-system   print the operating system
}

#[derive(Debug, Error)]
pub enum UnameError {}

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

pub fn cli_command(arg: &[String]) -> Result<(), UnameError> {
    let mut buf: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };

    unsafe {
        uname(&mut buf as *mut libc::utsname);
    };

    let opts = Opts::parse_from(arg);

    if opts.kernel_name
        || opts.all
        || (!opts.nodename && !opts.kernel_release && !opts.kernel_version && !opts.machine)
    {
        print_c_char(&buf.sysname);
        print!(" ");
    }

    if opts.nodename || opts.all {
        print_c_char(&buf.nodename);
        print!(" ");
    }

    if opts.kernel_release || opts.all {
        print_c_char(&buf.release);
        print!(" ");
    }

    if opts.kernel_version || opts.all {
        print_c_char(&buf.version);
        print!(" ");
    }

    if opts.machine || opts.all {
        print_c_char(&buf.machine);
        print!(" ");
    }

    println!();

    Ok(())
}
