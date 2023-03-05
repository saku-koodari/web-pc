use crate::utils::{
    bit_manipulation::get_bit_from_i16, convert_16b::from_i16, convert_bn::from_bool_array,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Register16BitEmulated {
    pub value: i16,
}

impl Register16BitEmulated {
    pub fn power_on() -> Self {
        Self { value: 0 }
    }

    pub fn power_on_with_state(value: i16) -> Self {
        Self { value }
    }

    pub fn mux(&self, input: [bool; 16], load: bool) -> [bool; 16] {
        let mut out = [false; 16];
        for i in 0..(16 as usize) {
            let current_value = input[i];
            let previous_value = get_bit_from_i16(self.value, i);

            out[i] = (!load && previous_value) || (load && current_value);
        }

        out
    }

    pub fn register_16bit_clocked(
        &mut self,
        input: [bool; 16],
        load: bool,
        clock: bool,
    ) -> [bool; 16] {
        if clock {
            self.value = from_bool_array(self.mux(input, load)).unwrap().as_integer;
        }

        from_i16(self.value).unwrap().as_array_b16
    }
}
