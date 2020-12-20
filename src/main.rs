mod bus;
mod cpu;
mod ppu;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut cpu = cpu::CPU::new();

    // Load ROM into memory
    let mut rom_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rom_path.push("rom/test_opcode.ch8");
    let mut rom = File::open(rom_path).unwrap();
    if let Ok(bytes_read) = rom.read(&mut cpu.ram[0x200..]) {
        bytes_read
    } else {
        0
    };

    for _ in 0..=10 {
        cpu.bus.PPU.canvas.present();
        cpu.evaluate_opcode();
    }
}
