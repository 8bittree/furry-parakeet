extern crate clap;
use clap::{Arg, App, crate_version};

use std::fs::read;

mod components;

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
        .get_matches();

    let mut mem = components::Memory::new(4096);

    if let Some(disk1_path) = matches.value_of("d1") {
        let disk1 = read(disk1_path)?;

        println!("disk1: {:?}", disk1.len());

        // check for boot signature
        if dbg!(disk1[510]) == 0x55 && dbg!(disk1[511]) == 0xAA {
            // (boot sector - boot signature) / 3 bytes per word
            // (512 - 2) / 3 = 170
            // ...then subtract 1 for indexing from 0
            for i in 0..169 {
                // little endian words
                let val = disk1[3*i+2] as u32
                    + ((disk1[3*i+1] as u32) << 8)
                    + ((disk1[3*i] as u32) << 16);
                mem[i].set(val);
            }
        }
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
