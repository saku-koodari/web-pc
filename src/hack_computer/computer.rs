use super::parts::{cpu::Cpu, memory::Memory, rom::Rom};

pub struct Computer {
    // parts
    cpu: Cpu,
    memory: Memory,
    rom: Rom,

    // buses
    cpu_data_bus: [bool; 16],
    instruction_address_bus: [bool; 16],

    // events
    pub reset: bool,
    pub clock: bool,
    pub screen_out: [bool; 16],
    pub keyboard_in: [bool; 16],
}

impl Computer {
    pub fn power_on() -> Self {
        Self {
            // power on parts
            cpu: Cpu::power_on(),
            memory: Memory::power_on(),
            rom: Rom::power_on(),

            // initialize buses
            cpu_data_bus: [false; 16],
            instruction_address_bus: [false; 16],

            // initialize events
            reset: false,
            clock: true,
            screen_out: [false; 16],
            keyboard_in: [false; 16],
        }
    }

    // separate events:
    // - reset
    // - clock
    // - screen out
    // - keyboard in

    /// iterates one cyckle of the computer
    pub fn run(&mut self) {
        // ROM
        let cpu_instr = self.rom.rom(self.instruction_address_bus);

        // CPU
        let (
            data_out_bus,            //
            write_enable,            //
            data_address_bus,        //
            instruction_address_bus, //
        ) = self
            .cpu
            .cpu(cpu_instr, self.cpu_data_bus, self.reset, self.clock);

        // Memory
        let (ram_out, screen_out, keyboard_out) = self.memory.memory(
            data_out_bus,     //
            write_enable,     //
            data_address_bus, //
            self.clock,
        );

        // update buses / events
        self.cpu_data_bus = ram_out;
        self.screen_out = screen_out;
        self.keyboard_in = keyboard_out;
    }
}

mod test {
    #[test]
    fn test_computer() {
        use super::*;
        let mut computer = Computer::power_on();

        computer.run();
    }
}
