/// The memory bus represents the NES's shared memory space
/// The CPU (and eventually the PPU) reads and writes bytes through this

#[allow(dead_code)]
#[derive(Debug)]
pub struct Bus {
    memory: [u8; 0xFFFF + 1],
}

#[allow(dead_code)]
impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF + 1],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read_returns_same_value() {
        let mut bus = Bus::new();
        bus.write(0x1234, 21);
        assert_eq!(bus.read(0x1234), 21);
    }
}
