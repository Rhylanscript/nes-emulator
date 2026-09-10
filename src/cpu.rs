use crate::bus::Bus;

/// the 6502 CPU: internal regs + bus
#[allow(dead_code)]
#[derive(Debug)]
pub struct Cpu {
    /// accumulator - main register for logic
    pub a: u8,
    /// x index register
    pub x: u8,
    /// y index register
    pub y: u8,
    /// stack pointer
    pub sp: u8,
    /// program counter - address of next instruction to exec
    pub pc: u16,
    /// status flags (carry, 0, neg, etc)
    pub status: u8,

    bus: Bus,
}

#[allow(dead_code)]
impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            status: 0,
            bus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cpu_has_expected_initial_state() {
        let bus = Bus::new();
        let cpu = Cpu::new(bus);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.pc, 0);
    }
}
