use super::chip8::Chip8;
use super::ppu::PPU;

#[allow(non_snake_case)]
pub struct Bus {
    CPU: Chip8,
    PPU: PPU,
    RAM: [u16; 4096],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            CPU: Chip8::new(),
            PPU: PPU::new(),
            RAM: [0x0000; 4096],
        }
    }
}
