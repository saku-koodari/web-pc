use crate::{
    emulated_parts::register_16bit_emulated::Register16BitEmulated,
    hack_computer::{
        chips::adder::inc16,
        gates::{gates_b1::or, gates_b16::mux16, gates_mw::mux4way16},
    },
};

pub struct ProgramCounterEmulated {
    base_circuit: Register16BitEmulated,
    feedback_out: [bool; 16],
}

impl ProgramCounterEmulated {
    pub fn power_on() -> Self {
        Self {
            base_circuit: Register16BitEmulated::power_on(),
            feedback_out: [false; 16],
        }
    }

    pub fn get_next_val(
        &mut self,
        input: [bool; 16],
        load: bool,
        inc: bool,
        reset: bool,
    ) -> [bool; 16] {
        // TODO: replace with muxes
        if reset {
            [false; 16]
        } else if load {
            input
        } else if inc {
            inc16(self.feedback_out)
        } else {
            self.feedback_out
        }
    }

    pub fn program_counter_clocked(
        &mut self,
        input: [bool; 16],
        load: bool,  // should emit input from the register on next cycle
        inc: bool,   // should emit inc16 from the register on next cycle
        reset: bool, // should emit zero from the register on next cycle
        clock: bool,
    ) -> [bool; 16] {
        // NOTE: EMULATED

        let reg_in = self.get_next_val(input, load, inc, reset);

        self.feedback_out = self
            .base_circuit
            .register_16bit_clocked(reg_in, true, clock);

        self.feedback_out
    }

    pub fn get_debug_info(&mut self) -> [bool; 16] {
        self.base_circuit.get_debug_info()
    }
}

// TODO: Write either better tests or panel for this.
// in order to test this meaningfully, you might need to consider mocking some of the child circuits
// or just write e2e tests
pub mod test {
    use super::*;

    #[test]
    fn test_register_16bit() {
        // one test is enough, basically just to test  that the struct is initalized correctly
        let mut register = ProgramCounterEmulated::power_on();

        let input = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];
        // let load = true;

        let load = false;
        let inc = false;
        let reset = false;
        // let clock = false;
        let mut clock = false;
        let mut output = register.program_counter_clocked(input, load, inc, reset, clock);
        assert_eq!(output, [false; 16], "tick tock 1");

        clock = true;
        output = register.program_counter_clocked(input, load, inc, reset, clock);

        assert_eq!(output, [false; 16], "tick tock 2");
    }
}
