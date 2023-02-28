use crate::hack_computer::{
    chips::flipflop_unsafe::dff_unsafe,
    gates::{
        gates_b1::{mux, or},
        gates_b16::mux16,
    },
};

use super::adder::inc16;

pub fn register_1bit_unsafe(input: bool, load: bool) -> bool {
    // Visual reprensentation of 1-BIT register
    //            load
    //              0
    //              |
    // in 0 ---- +--+--+          +-----+
    //           │ MUX ┝-mux_out--+ DFF +---┬-- out 0
    //       ┌--+------+          +-----+   |
    //       │                              |
    //       └------------------------------┘
    // As you can see from the visualization, these variables
    // are in order to make connection back to MUX gate from DFF gate.

    static mut OUT: bool = false;

    let mux_out = mux(input, unsafe { OUT }, load);
    unsafe {
        OUT = dff_unsafe(mux_out, load);
        return OUT;
    }
}

pub fn register_16bit_unsafe(input: [bool; 16], load: bool) -> [bool; 16] {
    let mut out = [false; 16];

    out[0] = register_1bit_unsafe(input[0], load);
    out[1] = register_1bit_unsafe(input[1], load);
    out[2] = register_1bit_unsafe(input[2], load);
    out[3] = register_1bit_unsafe(input[3], load);
    out[4] = register_1bit_unsafe(input[4], load);
    out[5] = register_1bit_unsafe(input[5], load);
    out[6] = register_1bit_unsafe(input[6], load);
    out[7] = register_1bit_unsafe(input[7], load);
    out[8] = register_1bit_unsafe(input[8], load);
    out[9] = register_1bit_unsafe(input[9], load);
    out[10] = register_1bit_unsafe(input[10], load);
    out[11] = register_1bit_unsafe(input[11], load);
    out[12] = register_1bit_unsafe(input[12], load);
    out[13] = register_1bit_unsafe(input[13], load);
    out[14] = register_1bit_unsafe(input[14], load);
    out[15] = register_1bit_unsafe(input[15], load);

    return out;
}

// TODO: Verify
pub fn program_counter_unsafe(input: [bool; 16], load: bool, inc: bool, reset: bool) -> [bool; 16] {
    let reset_out = mux16(input, [false; 16], reset);

    let load_or_reset = or(load, reset);

    static mut OUT: [bool; 16] = [false; 16];
    let reg_in = mux16(unsafe { OUT }, reset_out, load_or_reset);

    let reg_load = or(load, reset);
    let reg_out = register_16bit_unsafe(reg_in, reg_load);

    let inc_out = inc16(reg_out);

    unsafe {
        OUT = mux16(reg_out, inc_out, inc);

        OUT
    }
}
