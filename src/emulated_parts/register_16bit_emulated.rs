#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Register16BitEmulated {
    value: u16,
}

impl Register16BitEmulated {
    pub fn power_on() -> Self {
        Self { value: 0 }
    }

    pub fn power_on_with_state(value: u16) -> Self {
        Self { value }
    }

    fn u16_to_bn_arr<const N: usize>(num: u16) -> [bool; N] {
        let mut result = [false; N];
        for i in 0..N {
            result[15 - i] = (num >> i) & 1 == 1;
        }
        result
    }

    fn bn_to_u16<const N: usize>(arr: [bool; N]) -> u16 {
        let mut result = 0;
        for i in 0..N {
            result |= (arr[(N - 1) - i] as u16) << i;
        }
        result
    }

    pub fn register_16bit_clocked(
        &mut self,
        input: [bool; 16],
        load: bool,
        clock: bool,
    ) -> [bool; 16] {
        if load && clock {
            self.value = Self::bn_to_u16(input);
        }

        Self::u16_to_bn_arr(self.value)
    }
}
