extern crate clap;
use clap::{Arg, App, crate_version};

fn main() {
    println!("Hello, world!");

    let matches = App::new("furry-parakeet")
        .about("A shoddy VM for a poorly thought-out ISA.")
        .version(crate_version!())
        .arg(Arg::with_name("d1")
             .help("File to mount as Disk 1.")
             .long("d1")
             .visible_alias("disk1")
             .takes_value(true)
             .value_name("FILE"))
        .get_matches();
}
