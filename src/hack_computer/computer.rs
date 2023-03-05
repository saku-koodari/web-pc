use crate::{emulated_parts::rom_emulated::RomEmulated, utils::convert_16b::from_b16};

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
    pub fn power_on(rom_disk: Vec<i16>) -> Self {
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

    // DEBUG

    pub fn get_cpu_debug_info(&mut self) -> (i16, i16, i16) {
        fn to_i16(debug: &str, val: [bool; 16]) -> i16 {
            let cr = from_b16(val);
            match cr {
                Ok(n) => n.as_integer,
                Err(e) => {
                    println!("val: {:?}, Error in {}: {}", val, debug, e);
                    0
                }
            }
        }

        let cpu_info = self.cpu.get_debug_info();

        (
            to_i16("A register", cpu_info.0),
            to_i16("D register", cpu_info.1),
            to_i16("Program counter", cpu_info.2),
        )
    }

    pub fn get_ram(&self, start: usize, end: usize) -> Vec<(usize, i16)> {
        self.memory.get_ram(start, end)
    }

    pub fn print_cpu_debug_info(&mut self) { 
        // let cpu_info = self.get_cpu_debug_info();
    }

    pub fn print_ram(&mut self, start: usize, end: usize) {
       // self.get_ram(start, end)
    }
}

mod test {
    fn test_script() -> Vec<i16> {
        let code = [
            "6", "-5104", "0", "-7416", "7", "-5104", "1", "-7416", "0", "-5104", "2", "-7416",
            "0", "-5104", "3", "-7416", "3", "-1008", "1", "-2864", "31", "-7422", "0", "-1008",
            "2", "-3952", "-7416", "3", "-568", "16", "-5497", "2", "-1008", "31", "-5497",
        ];

        let mut script = Vec::new();

        for &str_value in code.iter() {
            let val: i16 = str_value.parse().unwrap_or_default();
            script.push(val);
        }

        script
    }

    #[test]
    fn test_computer() {
        use super::*;
        let rom_disk = test_script();
        let mut computer = Computer::power_on(rom_disk);

        for i in 0..35 {
            println!("cycle: {}", i);

            computer.run();
            computer.print_cpu_debug_info();
            computer.print_ram(0, 40);
        }

        panic!("test!")
    }
}
