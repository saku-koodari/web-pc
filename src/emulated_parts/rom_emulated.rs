use crate::emulated_parts::register_16bit_emulated::Register16BitEmulated;

pub struct RomEmulated {
    registers: [Register16BitEmulated; 32768],
}

impl RomEmulated {
    pub fn power_on(disk: [i16; 32768]) -> Self {
        let mut registers = [Register16BitEmulated::power_on_with_state(0); 32768];
        for i in 0..32768 {
            registers[i] = Register16BitEmulated::power_on_with_state(disk[i]);
        }
        Self { registers }
    }

    // receives the instruction address bus
    // returns the instruction for the CPU
    pub fn rom(&mut self, instruction: [bool; 16]) -> [bool; 16] {
        [false; 16]
    }
}
