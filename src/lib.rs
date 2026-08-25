pub const NOP: u8 = 1;

#[derive(Debug)]
pub struct Cpu {
    pub pc: u16,
    pub mem: [u8; 65536],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            pc: 0,
            mem: [0; 65536],
        }
    }
}

impl Cpu {
    pub fn step(&mut self) {
        // Over to you to implement `step`!
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_correctly_initialises_cpu() {
        let cpu = Cpu::default();
        assert_eq!(cpu.pc, 0, "wrong initial PC");
        assert_eq!(*cpu.mem.first().unwrap(), 0, "wrong memory contents");
    }

    // Uncomment this test once the previous test passes!
    // #[test]
    // fn nop_instruction_increments_pc() {
    //     let mut cpu = Cpu::default();
    //     cpu.mem[256] = NOP;
    //     cpu.pc = 256;
    //     cpu.step();
    //     assert_eq!(cpu.pc, 257, "wrong PC after step()")
    // }
}
