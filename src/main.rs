use crate::{bus::Bus, cpu::Cpu};

mod bus;
mod cpu;

fn main() {
    let bus = Bus::new();
    let cpu = Cpu::new(bus);

    println!("CPU initialized: {:?}", cpu);
}
