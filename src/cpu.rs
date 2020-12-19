#[allow(non_snake_case)]
pub struct Chip8 {
    // Registers
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    /********************************************/
    // Shouldn't be set by programs, used by certain instructions
    VF: u8,
    /********************************************/
    // Stores memory addresses
    I: u16,
    /********************************************/
    // Usually zero, decrement by 60hz when not
    DT: u8,
    ST: u8,
    /********************************************/
    // Program Counter
    PC: u16,
    // Stack Pointer
    SP: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            V1: 0x00,
            V2: 0x00,
            V3: 0x00,
            V4: 0x00,
            V5: 0x00,
            V6: 0x00,
            V7: 0x00,
            V8: 0x00,
            V9: 0x00,
            VA: 0x00,
            VB: 0x00,
            VC: 0x00,
            VD: 0x00,
            VE: 0x00,
            VF: 0x00,
            I: 0x0000,
            DT: 0x00,
            ST: 0x00,
            PC: 0x0000,
            SP: 0x00,
        }
    }

    // Return from subroutine
    pub fn RET(&mut self) {
        self.PC = self.SP as u16;
        self.SP -= 1;
    }

    pub fn JP(&mut self, addr: u16) {
        self.PC = addr;
    }

    pub fn CALL(&mut self, addr: u16) {}
}
