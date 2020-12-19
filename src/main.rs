mod bus;
mod cpu;
mod ppu;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut bus = bus::Bus::new();

    // Load ROM into memory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("rom/test_opcode.ch8");
    let mut rom = File::open(path).unwrap();
    if let Ok(bytes_read) = rom.read(&mut bus.RAM[0x200..]) {
        bytes_read
    } else {
        0
    };
}
