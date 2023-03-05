use super::register_1bit::Register1Bit;

pub struct Register16Bit {
    child_circuits: [Register1Bit; 16],
    feedback_out: [bool; 16],
}

impl Register16Bit {
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
            feedback_out: [false; 16],
        }
    }

    pub fn register_16bit_clocked(
        &mut self,
        input: [bool; 16],
        load: bool,
        clock: bool,
    ) -> [bool; 16] {
        self.feedback_out[0] = self.child_circuits[0].register_1bit_clocked(input[0], load, clock);
        self.feedback_out[1] = self.child_circuits[1].register_1bit_clocked(input[1], load, clock);
        self.feedback_out[2] = self.child_circuits[2].register_1bit_clocked(input[2], load, clock);
        self.feedback_out[3] = self.child_circuits[3].register_1bit_clocked(input[3], load, clock);
        self.feedback_out[4] = self.child_circuits[4].register_1bit_clocked(input[4], load, clock);
        self.feedback_out[5] = self.child_circuits[5].register_1bit_clocked(input[5], load, clock);
        self.feedback_out[6] = self.child_circuits[6].register_1bit_clocked(input[6], load, clock);
        self.feedback_out[7] = self.child_circuits[7].register_1bit_clocked(input[7], load, clock);
        self.feedback_out[8] = self.child_circuits[8].register_1bit_clocked(input[8], load, clock);
        self.feedback_out[9] = self.child_circuits[9].register_1bit_clocked(input[9], load, clock);
        self.feedback_out[10] =
            self.child_circuits[10].register_1bit_clocked(input[10], load, clock);
        self.feedback_out[11] =
            self.child_circuits[11].register_1bit_clocked(input[11], load, clock);
        self.feedback_out[12] =
            self.child_circuits[12].register_1bit_clocked(input[12], load, clock);
        self.feedback_out[13] =
            self.child_circuits[13].register_1bit_clocked(input[13], load, clock);
        self.feedback_out[14] =
            self.child_circuits[14].register_1bit_clocked(input[14], load, clock);
        self.feedback_out[15] =
            self.child_circuits[15].register_1bit_clocked(input[15], load, clock);

        return self.feedback_out;
    }

    pub fn get_debug_info(&mut self) -> [bool; 16] {
        let mut res = [false; 16];
        for i in 0..16 {
            res[i] = self.child_circuits[i].current_value;
        }

        res
    }
}

pub mod test {
    use super::*;

    #[test]
    fn test_register_16bit() {
        // one test is enough, basically just to test  that the struct is initalized correctly
        let mut register = Register16Bit::power_on();

        let input = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];
        let load = true;
        let mut clock = false;
        let mut output = register.register_16bit_clocked(input, load, clock);
        assert_eq!(output, [false; 16], "tick tock 1");

        clock = true;
        output = register.register_16bit_clocked(input, load, clock);

        assert_eq!(output, [false; 16], "tick tock 2");
    }
}
