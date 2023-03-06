use crate::{
    emulated_parts::register_16bit_emulated::Register16BitEmulated,
    utils::convert_16b::{from_b16, from_i16},
};

pub struct RomEmulated {
    registers: [Register16BitEmulated; 32768],
}

impl RomEmulated {
    pub fn power_on(disk: Vec<i16>) -> Self {
        let mut registers = [Register16BitEmulated::power_on_with_state(0); 32768];
        for i in 0..disk.len() {
            registers[i] = Register16BitEmulated::power_on_with_state(disk[i]);
        }
        Self { registers }
    }

    // receives the instruction address bus
    // returns the instruction for the CPU
    pub fn rom(&mut self, instruction: [bool; 16]) -> [bool; 16] {
        let u_instruction = from_b16(instruction).unwrap().as_usize;
        let reg_value = self.registers[u_instruction];

        from_i16(reg_value.value).unwrap().as_array_b16
    }
}
