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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_correctly_initialises_cpu() {
        let cpu = Cpu::default();
        assert_eq!(cpu.pc, 0, "wrong initial PC");
        assert_eq!(*cpu.mem.first().unwrap(), 0, "wrong memory contents");
    }
}
