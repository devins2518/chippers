use super::cpu::CPU;
use super::ppu::PPU;

#[allow(non_snake_case)]
pub struct Bus {
    pub CPU: CPU,
    pub PPU: PPU,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            CPU: CPU::new(),
            PPU: PPU::new(),
        }
    }
}
