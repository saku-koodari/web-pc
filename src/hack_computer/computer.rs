use crate::emulated_parts::rom_emulated::RomEmulated;

use super::parts::{cpu::Cpu, memory::Memory};

pub struct Computer {
    // parts
    cpu: Cpu,
    memory: Memory,
    rom: RomEmulated,

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
    pub fn power_on(rom_disk: [u16; 32768]) -> Self {
        Self {
            // power on parts
            cpu: Cpu::power_on(),
            memory: Memory::power_on(),
            rom: RomEmulated::power_on(rom_disk),

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

    pub fn get_input_from_io_device(&mut self, input: [bool; 16], clock: bool) {
        self.memory.write_from_io_driver(input, clock);
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
        let ram_out = self.memory.memory(
            data_out_bus,     //
            write_enable,     //
            data_address_bus, //
            self.clock,
        );

        // update buses / events
        self.cpu_data_bus = ram_out;
        self.instruction_address_bus = [
            instruction_address_bus[0],
            instruction_address_bus[1],
            instruction_address_bus[2],
            instruction_address_bus[3],
            instruction_address_bus[4],
            instruction_address_bus[5],
            instruction_address_bus[6],
            instruction_address_bus[7],
            instruction_address_bus[8],
            instruction_address_bus[9],
            instruction_address_bus[10],
            instruction_address_bus[11],
            instruction_address_bus[12],
            instruction_address_bus[13],
            instruction_address_bus[14],
            false,
        ];
    }
}

mod test {
    fn test_script() -> [u16; 32768] {
        let code = [
            "6", "-5104", "0", "-7416", "7", "-5104", "1", "-7416", "0", "-5104", "2", "-7416",
            "0", "-5104", "3", "-7416", "3", "-1008", "1", "-2864", "31", "-7422", "0", "-1008",
            "2", "-3952", "-7416", "3", "-568", "16", "-5497", "2", "-1008", "31", "-5497",
        ];

        let mut script: [u16; 32768] = [0; 32768];
        let mut i = 0;
        for line in code {
            let mut line = line.chars().as_str();
            let mut instruction = [false; 16];

            let res = u16::from_str_radix(line, 10);
            script[i] = match res {
                Ok(n) => n,
                Err(_) => 0,
            }
        }

        script
    }

    #[test]
    fn test_computer() {
        use super::*;
        let rom_disk = test_script();
        let mut computer = Computer::power_on(rom_disk);

        computer.run();
    }
}
