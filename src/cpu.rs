use super::bus::Bus;

#[allow(non_snake_case)]
pub struct CPU {
    // Registers
    pub V: [u8; 16],
    /********************************************/
    // Stores memory addresses
    pub I: u16,
    /********************************************/
    // Usually zero, decrement by 60hz when not
    pub DT: u8,
    pub ST: u8,
    /********************************************/
    // Program Counter
    pub PC: u16,
    // Stack Pointer
    pub SP: u8,
    pub stack: [u16; 16],
    pub ram: [u8; 4096],
    pub bus: Bus,
}

#[allow(non_snake_case)]
impl CPU {
    pub fn new() -> Self {
        let mut ram = [0x00; 4096];
        ram[0..80].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, /* 0 */
            0x20, 0x60, 0x20, 0x20, 0x70, /* 1 */
            0xF0, 0x10, 0xF0, 0x80, 0xF0, /* 2 */
            0xF0, 0x10, 0xF0, 0x10, 0xF0, /* 3 */
            0x90, 0x90, 0xF0, 0x10, 0x10, /* 4 */
            0xF0, 0x80, 0xF0, 0x10, 0xF0, /* 5 */
            0xF0, 0x80, 0xF0, 0x90, 0xF0, /* 6 */
            0xF0, 0x10, 0x20, 0x40, 0x40, /* 7 */
            0xF0, 0x90, 0xF0, 0x90, 0xF0, /* 8 */
            0xF0, 0x90, 0xF0, 0x10, 0xF0, /* 9 */
            0xF0, 0x90, 0xF0, 0x90, 0x90, /* A */
            0xE0, 0x90, 0xE0, 0x90, 0xE0, /* B */
            0xF0, 0x80, 0x80, 0x80, 0xF0, /* C */
            0xE0, 0x90, 0x90, 0x90, 0xE0, /* D */
            0xF0, 0x80, 0xF0, 0x80, 0xF0, /* E */
            0xF0, 0x80, 0xF0, 0x80, 0x80, /* F */
        ]);
        CPU {
            V: [0; 16],
            I: 0x0000,
            DT: 0x00,
            ST: 0x00,
            PC: 0x200,
            // Points to topmost level of stack
            SP: 0xFF,
            stack: [0x0000; 16],
            ram,
            bus: Bus::new(),
        }
    }

    // Basically copied from
    // https://github.com/ColinEberhardt/wasm-rust-chip8/blob/master/src/cpu.rs
    pub fn evaluate_opcode(&mut self) {
        let opcode = self.read_memory(self.PC);

        let nnn = opcode & 0x0FFF;
        let x = opcode & 0x0F00;
        let y = opcode & 0x00F0;
        let kk = opcode & 0x00FF;
        let n = opcode & 0x000F;

        let op1 = opcode & 0xF000;
        let op2 = opcode & 0x0F00;
        let op3 = opcode & 0x00F0;
        let op4 = opcode & 0x000F;

        println!(
            "nnn {} x {} y {} kk {} n {}\nop1 {} op2 {} op3 {} op4 {}",
            nnn, x, y, kk, n, op1, op2, op3, op4
        );

        // Switching to function pointer table will be much faster
        match (op1, op2, op3, op4) {
            // Ignore SYS call
            (0x0, 0x0, 0xE, 0x0) => self.bus.PPU.CLS(),
            (0x0, 0x0, 0xE, 0xE) => self.RET(),
            (0x1, _, _, _) => self.JP(nnn),

            _ => (),
        }

        self.PC += 2;
    }

    // u16 because of PC size
    pub fn read_memory(&self, addr: u16) -> u16 {
        /* Take value at first byte as first nibble */
        /* Add value at second byte as second nibble */
        (self.ram[addr as usize] as u16) << 8 | (self.ram[addr as usize + 1] as u16)
    }

    pub fn write_memory(&mut self, addr: u8, value: u8) {
        self.ram[addr as usize] = value;
    }

    // Return from subroutine
    pub fn RET(&mut self) {
        self.PC = self.SP as u16;
        self.SP -= 1;
    }

    pub fn JP(&mut self, addr: u16) {
        self.PC = addr;
    }

    pub fn CALL(&mut self, addr: u16) {
        todo!();
    }

    pub fn SE(&mut self, addr: u16) {
        todo!();
    }

    pub fn SNE(&mut self, addr: u16) {
        todo!();
    }

    pub fn LD(&mut self, addr: u16) {
        todo!();
    }

    pub fn ADD(&mut self, addr: u16) {
        todo!();
    }

    pub fn OR(&mut self, addr: u16) {
        todo!();
    }

    pub fn AND(&mut self, addr: u16) {
        todo!();
    }

    pub fn XOR(&mut self, addr: u16) {
        todo!();
    }

    pub fn SUB(&mut self, addr: u16) {
        todo!();
    }

    pub fn SHR(&mut self, addr: u16) {
        todo!();
    }

    pub fn SUBN(&mut self, addr: u16) {
        todo!();
    }

    pub fn SHL(&mut self, addr: u16) {
        todo!();
    }

    pub fn RND(&mut self, addr: u16) {
        todo!();
    }

    pub fn DRW(&mut self, addr: u16) {
        todo!();
    }

    pub fn SKP(&mut self, addr: u16) {
        todo!();
    }

    pub fn SKNP(&mut self, addr: u16) {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mem_read() {
        let cpu = CPU::new();

        assert_eq!(cpu.read_memory(0x00), 0xF090);
        assert_eq!(cpu.read_memory(0x01), 0x9090);
        assert_eq!(cpu.read_memory(0x02), 0x9090);
        assert_eq!(cpu.read_memory(0x03), 0x90F0);
        assert_eq!(cpu.read_memory(0x04), 0xF020);
        assert_eq!(cpu.read_memory(0x05), 0x2060);
        assert_eq!(cpu.read_memory(0x06), 0x6020);
        assert_eq!(cpu.read_memory(0x07), 0x2020);
        assert_eq!(cpu.read_memory(0x08), 0x2070);
        assert_eq!(cpu.read_memory(0x09), 0x70F0);
    }

    #[test]
    fn test_mem_write() {
        let mut cpu = CPU::new();
        cpu.write_memory(0x00, 0xFF);
        cpu.write_memory(0x01, 0xFF);
        cpu.write_memory(0x02, 0xFF);
        cpu.write_memory(0x03, 0xFF);
        cpu.write_memory(0x04, 0xFF);
        cpu.write_memory(0x05, 0xFF);
        cpu.write_memory(0x06, 0xFF);
        cpu.write_memory(0x07, 0xFF);
        cpu.write_memory(0x08, 0xFF);
        cpu.write_memory(0x09, 0xFF);

        assert_eq!(cpu.read_memory(0x00), 0xFFFF);
        assert_eq!(cpu.read_memory(0x01), 0xFFFF);
        assert_eq!(cpu.read_memory(0x02), 0xFFFF);
        assert_eq!(cpu.read_memory(0x03), 0xFFFF);
        assert_eq!(cpu.read_memory(0x04), 0xFFFF);
        assert_eq!(cpu.read_memory(0x05), 0xFFFF);
        assert_eq!(cpu.read_memory(0x06), 0xFFFF);
        assert_eq!(cpu.read_memory(0x07), 0xFFFF);
        assert_eq!(cpu.read_memory(0x08), 0xFFFF);
        assert_eq!(cpu.read_memory(0x09), 0xFFF0);
    }
}
