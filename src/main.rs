extern crate clap;
use clap::{Arg, App, crate_version};

use std::fs::read;

mod components;

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

    let mut mem = components::Memory::new(4096);

    let rom;

    if let Some(rom_path) = matches.value_of("rom") {
        rom = read(rom_path)?;
    } else {
        rom = vec![];
    }

    if rom.len() != ROM_BYTES {
        panic!("Malformed ROM");
    }

    for i in 0..255 {
        // little endian words
        let val = rom[3*i+2] as u32
            + ((rom[3*i+1] as u32) << 8)
            + ((rom[3*i] as u32) << 16);
        mem[i].set(val);
    }

    println!("debug:   {:?}", mem[0]);
    println!("alt:     {:#?}", mem[0]);

    println!("display: {}", mem[0]);
    println!("alt:     {:#}", mem[0]);
    println!("pad:     {:4}", mem[0]);
    println!("lhex:    {:x}", mem[0]);
    println!("uhex:    {:#X}", mem[0]);
    println!("octal:   {:08o}", mem[0]);
    println!("bin:     {:024b}", mem[0]);

    Ok(())
}
