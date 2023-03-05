use super::conv::{bn_to_i16, i16_to_bn_arr, get_bit_from_i16};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Register16BitEmulated {
    value: i16,
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
            out[i] = (!load && get_bit_from_i16(self.value, i)) || (load && input[i]);
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
            self.value = bn_to_i16(self.mux(input, load));
        }

        i16_to_bn_arr(self.value)
    }
}
