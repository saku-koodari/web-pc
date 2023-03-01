use crate::hack_computer::gates::gates_b1::not;

use super::latch::Latch;

pub struct FlipFlop {
    child_circuit: [Latch; 2],
}

impl FlipFlop {
    pub fn power_on() -> Self {
        Self {
            child_circuit: [
                Latch {
                    prev_q_high: false,
                    prev_q_low: true,
                },
                Latch {
                    prev_q_high: false,
                    prev_q_low: true,
                },
            ],
        }
    }

    /// Flipflop, 0 is out, 1 and 2 are for debugging
    pub fn flipflop(&mut self, data: bool, clock: bool) -> (bool, bool, bool) {
        // Q low is not needed, you might want to use it for debugging
        let (latch1_out, latch1_q_low) = self.child_circuit[0].d_latch(data, not(clock));
        let (latch2_out, latch2_q_low) = self.child_circuit[1].d_latch(latch1_out, clock);

        (latch2_out, latch1_q_low, latch2_q_low)
    }
}
