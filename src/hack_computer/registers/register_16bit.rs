use super::register_1bit::Register1Bit;

pub struct Register16Bit {
    child_circuits: [Register1Bit; 16],
    feedback_out: [bool; 16],
}

/*
impl Register16Bit {
    pub fn power_on() -> Self {
        // I need to loop this 16 times
        let one_bit = Register1Bit {
            child_circuit: Latch {
                prev_q_high: false,
                prev_q_low: true,
            },
            feedback_out: false,
            child_circuit_v2: [
                Latch {
                    prev_q_high: false,
                    prev_q_low: true,
                },
                Latch {
                    prev_q_high: false,
                    prev_q_low: true,
                },
            ],
        };

        Self {
            child_circuits: [Register1Bit::clone(); 16],
            feedback_out: [false; 16],
        }
    }

    pub fn register_16bit(&mut self, input: [bool; 16], load: bool) -> [bool; 16] {
        self.feedback_out[0] = self.child_circuits[0].register_1bit(input[0], load);
        self.feedback_out[1] = self.child_circuits[1].register_1bit(input[1], load);
        self.feedback_out[2] = self.child_circuits[2].register_1bit(input[2], load);
        self.feedback_out[3] = self.child_circuits[3].register_1bit(input[3], load);
        self.feedback_out[4] = self.child_circuits[4].register_1bit(input[4], load);
        self.feedback_out[5] = self.child_circuits[5].register_1bit(input[5], load);
        self.feedback_out[6] = self.child_circuits[6].register_1bit(input[6], load);
        self.feedback_out[7] = self.child_circuits[7].register_1bit(input[7], load);
        self.feedback_out[8] = self.child_circuits[8].register_1bit(input[8], load);
        self.feedback_out[9] = self.child_circuits[9].register_1bit(input[9], load);
        self.feedback_out[10] = self.child_circuits[10].register_1bit(input[10], load);
        self.feedback_out[11] = self.child_circuits[11].register_1bit(input[11], load);
        self.feedback_out[12] = self.child_circuits[12].register_1bit(input[12], load);
        self.feedback_out[13] = self.child_circuits[13].register_1bit(input[13], load);
        self.feedback_out[14] = self.child_circuits[14].register_1bit(input[14], load);
        self.feedback_out[15] = self.child_circuits[15].register_1bit(input[15], load);

        return self.feedback_out;
    }
}

*/
