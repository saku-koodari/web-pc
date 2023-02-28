use crate::hack_computer::{
    chips::{flipflop::LatchCircuit, adder::inc16},
    gates::{
        gates_b1::{mux, or},
        gates_b16::mux16,
    },
};

use super::register_16bit::Register16Bit;

#[derive(Clone, Copy)]
pub struct ProgramCounter {
    base_circuit: Register16Bit,
    feedback_out: [bool; 16],
}

impl ProgramCounter {
    pub fn power_on() -> Self {
        Self {
            base_circuit: Register16Bit::power_on(),
            feedback_out: [false; 16],
        }
    }
    pub fn progam_counter(
        &mut self,
        input: [bool; 16],
        load: bool,
        inc: bool,
        reset: bool,
    ) -> [bool; 16] {
        let reset_out = mux16(input, [false; 16], reset);
        let load_or_reset = or(load, reset);

        let reg_in = mux16(self.feedback_out, reset_out, load_or_reset);

        let reg_load = or(load, reset);
        let reg_out = self.base_circuit.register_16bit(reg_in, reg_load);

        let inc_out = inc16(reg_out);
        self.feedback_out = mux16(reg_out, inc_out, inc);

        self.feedback_out
    }
}
