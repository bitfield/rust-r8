#[derive(Debug)]
pub struct Cpu {
    pub pc: usize,
    pub mem: [usize; 256],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            pc: 0,
            mem: [0; 256],
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
    // fn step_increments_pc() {
    //     let mut cpu = Cpu::default();
    //     cpu.mem[0] = 1;
    //     cpu.step();
    //     assert_eq!(cpu.pc, 1, "wrong PC after step()")
    // }
}
