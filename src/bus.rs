use super::ppu::PPU;

#[allow(non_snake_case)]
pub struct Bus {
    pub PPU: PPU,
}

impl Bus {
    pub fn new() -> Self {
        Bus { PPU: PPU::new() }
    }
}
