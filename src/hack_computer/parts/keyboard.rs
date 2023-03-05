use crate::hack_computer::registers::register_1bit::Register1Bit;

pub struct Keyboard {
    child_circuits: [Register1Bit; 16],
    values: [bool; 16],
}

impl Keyboard {
    pub fn power_on() -> Self {
        Self {
            child_circuits: [
                // TODO: use loop or some kind of cloning
                // I just did it like this, because this work and cause no memory issues.
                Register1Bit::power_on(), // 1
                Register1Bit::power_on(), // 2
                Register1Bit::power_on(), // 3
                Register1Bit::power_on(), // 4
                Register1Bit::power_on(), // 5
                Register1Bit::power_on(), // 6
                Register1Bit::power_on(), // 7
                Register1Bit::power_on(), // 8
                Register1Bit::power_on(), // 9
                Register1Bit::power_on(), // 10
                Register1Bit::power_on(), // 11
                Register1Bit::power_on(), // 12
                Register1Bit::power_on(), // 13
                Register1Bit::power_on(), // 14
                Register1Bit::power_on(), // 15
                Register1Bit::power_on(), // 16
            ],
            values: [false; 16],
        }
    }

    pub fn write(&mut self, input: [bool; 16], clock: bool){
        self.values = self.register_nbit_clocked::<16>(input, true, clock)
    }


    /// Register 16 bit
    /// Rwister count: 1
    pub fn read(&mut self, clock: bool) -> [bool; 16] {
        self.register_nbit_clocked::<16>(self.values, false, clock)
    }

    fn register_nbit_clocked<const N: usize>(
        &mut self,
        input: [bool; N],
        load: bool,
        clock: bool,
    ) -> [bool; N] {
        let mut output = [false; N];
        for i in 0..N {
            output[i] = self.child_circuits[i]
                .register_1bit_clocked(input[i], load, clock);
        }

        return output
    }
}
