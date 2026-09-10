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
            0xAD => self.lda_absolute(),
            0x8D => self.sta_absolute(),
            0x69 => self.adc_immediate(),
            _ => panic!("Unimplemented opcode: {:#04X}", opcode),
        }
    }

    /// LDA (immediate): load the next byte directly into accumulator
    fn lda_immediate(&mut self) {
        let value = self.fetch_byte();
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    /// LDA (absolute): load the value stored at a 16b addr into accumulator
    fn lda_absolute(&mut self) {
        let addr = self.fetch_word();
        let value = self.bus.read(addr);
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    /// STA (absolute): store the accumulators current value into memory at a 16b addr
    fn sta_absolute(&mut self) {
        let addr = self.fetch_word();
        self.bus.write(addr, self.a);
    }

    /// ADC (immediate): add next byte plus current carry flag to accumulator
    fn adc_immediate(&mut self) {
        let value = self.fetch_byte();
        let carry_in: u8 = if self.status & 0b0000_0001 != 0 { 1 } else { 0 };

        let sum = self.a as u16 + value as u16 + carry_in as u16;

        if sum > 0xFF {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }

        self.a = sum as u8;
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

    /// reads 2 bytes starting at current pc and combines
    /// them into 16b addr
    fn fetch_word(&mut self) -> u16 {
        let low = self.fetch_byte() as u16;
        let high = self.fetch_byte() as u16;
        (high << 8) | low
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

    #[test]
    fn lda_absolute_loads_value_from_memory_address() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0xAD);
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0x00);
        bus.write(0x0042, 0x99);
        let mut cpu = Cpu::new(bus);

        cpu.step();

        assert_eq!(cpu.a, 0x99);
    }

    #[test]
    fn lda_absolute_sets_negative_flag_when_high_bit_set() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0xAD);
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0x00);
        bus.write(0x0042, 0b1000_0001);
        let mut cpu = Cpu::new(bus);

        cpu.step();

        assert_eq!(cpu.a, 0b1000_0001);
        assert_ne!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn sta_absolute_writes_accumulator_to_memory() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0x8D);
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0x00);
        let mut cpu = Cpu::new(bus);
        cpu.a = 0x99;

        cpu.step();

        assert_eq!(cpu.bus.read(0x0042), 0x99);
    }

    #[test]
    fn sta_absolute_does_not_affect_status_flags() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0x8D);
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0x00);
        let mut cpu = Cpu::new(bus);
        cpu.a = 0x00;
        cpu.status = 0b1111_1111;

        cpu.step();

        assert_eq!(cpu.status, 0b1111_1111);
    }

    #[test]
    fn adc_immediate_adds_value_to_accumulator() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0x69);
        bus.write(0x0001, 0x10);
        let mut cpu = Cpu::new(bus);
        cpu.a = 0x05;

        cpu.step();

        assert_eq!(cpu.a, 0x15);
        assert_eq!(cpu.status & 0b0000_0001, 0);
    }

    #[test]
    fn adc_immediate_includes_existing_carry_flag() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0x69);
        bus.write(0x0001, 0x10);
        let mut cpu = Cpu::new(bus);
        cpu.a = 0x05;
        cpu.status |= 0b0000_0001;

        cpu.step();

        assert_eq!(cpu.a, 0x16);
    }

    #[test]
    fn adc_immediate_sets_carry_flag_on_overflow() {
        let mut bus = Bus::new();
        bus.write(0x0000, 0x69);
        bus.write(0x0001, 0x01);
        let mut cpu = Cpu::new(bus);
        cpu.a = 0xFF;

        cpu.step();

        assert_eq!(cpu.a, 0x00);
        assert_ne!(cpu.status & 0b0000_0001, 0);
        assert_ne!(cpu.status & 0b0000_0010, 0);
    }
}
