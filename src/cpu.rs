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

    /// Reads 1 byte from the bus at the current program counter
    /// then advances the pc by 1
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    /// runs a single fetch-decode-exec step
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
            0xA9 => self.lda_immediate(),
            _ => panic!("Unimplemented opcode: {:#04X}", opcode),
        }
    }

    /// LDA (immediate): load the next byte directly into accumulator
    fn lda_immediate(&mut self) {
        let value = self.fetch_byte();
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    /// Updates the zero and negative status flags based on a result value
    /// these 2 flags get updated after every instruction
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // 0 flag (bit 1): set if result was 0
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        // neg flag (bit 7): set if bit of result is set
        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
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

    #[test]
    fn lda_immediate_loads_value_into_accumulator() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0xA9);
        bus.write(0x0001, 0x42);
        let mut cpu = Cpu::new(bus);

        cpu.step();

        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn lda_immediate_sets_zero_flag_when_loading_zero() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0xA9);
        bus.write(0x0001, 0x00);
        let mut cpu = Cpu::new(bus);

        cpu.step();

        assert_eq!(cpu.a, 0x00);
        assert_ne!(cpu.status & 0b0000_0010, 0);
    }
}
