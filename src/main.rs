extern crate clap;
use clap::{Arg, App, crate_version};

use std::fs::read;

mod components;
mod machine;

/// Size of a ROM
///
/// 256 words = 768 bytes
const ROM_BYTES: usize = 768;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg(Arg::with_name("rom")
            .help("ROM file to load.")
            .long("rom")
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    let rom;

    if let Some(rom_path) = matches.value_of("rom") {
        rom = read(rom_path)?;
    } else {
        rom = vec![];
    }

    if rom.len() != ROM_BYTES {
        panic!("Malformed ROM");
    }

    let mut vm = machine::Machine::new(components::DEFAULT_MEM_SIZE, rom);
    vm.start();

    println!("{}", vm);

    Ok(())
}
