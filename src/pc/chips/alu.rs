use crate::pc::gates::gates_b16::{mux16, not16};

// our ALU can't do multiplication or division
// they will be implemented on the software level.
// However, that will be a trade-off between speed and having more "hardware".
pub fn alu(
    x: [bool; 16], // 16-bit input x
    y: [bool; 16], // 16-bit input y
    zx: bool,      // zero the x input?
    nx: bool,      // negate the x input?
    zy: bool,      // zero the y input?
    ny: bool,      // negate the y input?
    f: bool,       // function selector
    no: bool,      // negate the output?
) -> (
    [bool; 16], // out
    bool,       // zr: zero result
    bool,       // ng: negative result
) {
    // Manipulates the x and y inputs as follows:
    // if (zx == 1)  sets x = 0       // 16-bit true constant
    let opcode_reset_x = mux16(x, [false; 16], zx);

    // if (nx == 1)  sets x = !x      // bitwise NOT
    let opcode_negator_x = mux16(opcode_reset_x, not16(x), nx);

    // if (zy == 1)  sets y = 0       // 16-bit true constant
    let opcode_reset_y = mux16(y, [false; 16], zy);

    // if (ny == 1)  sets y = !y      // bitwise NOT
    let opcode_negator_y = mux16(opcode_reset_y, not16(y), ny);

    // if (f == 1)   sets out = x + y // int. 2's s-complement addition
    // let res = mux16(  f)

    // if (f == 0)   sets out = x & y // bitwise AND
    // if (no == 1)  sets out = !out  // bitwise NOT
    // if (out == 0) sets zr = 1      // 1-bit true constant
    // if (out < 0)  sets ng = 1      // 1-bit true constant

    // Note: Try to implement the chips in the given order
    // Note: should not be more than ~20-30 lines of code
    // Note: strive to use as few chip-parts as possible
    // Note: consider adding debug functions and drivers to UI
    // in order to test this in the client.

    panic!("Not implemented yet!");
}

mod tests {
    // unit tests for ALU (arithmatic logic unit)
    // use super::*;
    //
    // meaning of the control bits:
    // zx: set x to zero
    // nx: negates x
    // zy: set y to zero
    // ny: negates y
    // f: function selector
    // out: output of the function
    //
    // preset-x: pre-setting the x input
    // preset-y: pre-setting the y input
    // sel: selecting between computing + or &
    // post: post-setting the output
    // out: Reslting ALU output
    // -----------------------------------
    // zx:  if zx then x=0
    // nx:  if nx then x=!x
    // zy:  if zy then y=0
    // ny:  if ny then y=!y
    // f:   if f then out=x+y else out=x&y
    // no:  if no then out=!out
    // out: out(x,y)=
    // -----------------------------------
    // ALU also outputs zr and ng
    // zr: if out=0 then zr=1 else zr=0
    // ng: if out<0 then ng=1 else ng=0

    // the truth table for the ALU

    // +----+-----------------------------------+-----+
    // |    |            control bits           | out |
    // +----+-----------------------------------+-----+
    // |    | preset-x  | preset-y  | sel |post | out |
    // +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+
    // | ## | zx  | nx  | zy  | ny  |  f  | no  | out | => |  6-bit = opcodes | instr |
    // +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+
    // | 01 |  1  |  0  |  1  |  0  |  1  |  0  |  0  |    | 101010 =    2A   | reset |
    // | 02 |  1  |  1  |  1  |  1  |  1  |  1  |  1  |    | 111111 =    3F   | fill  |
    // | 03 |  1  |  1  |  1  |  0  |  1  |  0  | -1  |    | 111010 =    3E   | neg   |
    // | 04 |  0  |  0  |  1  |  1  |  0  |  0  |  x  |    | 001100 =    0C   | x     |
    // | 05 |  1  |  1  |  0  |  0  |  0  |  0  |  y  |    | 110000 =    30   | y     |
    // | 06 |  0  |  0  |  1  |  1  |  0  |  1  | !x  |    | 001101 =    0D   | negx  |
    // | 07 |  1  |  1  |  0  |  0  |  0  |  1  | !y  |    | 110001 =    31   | negy  |
    // | 08 |  0  |  0  |  1  |  1  |  1  |  1  | -x  |    | 001111 =    0F   | xsub  |
    // | 09 |  1  |  1  |  0  |  0  |  1  |  1  | -y  |    | 110011 =    33   | ysub  |
    // | 10 |  0  |  1  |  1  |  1  |  1  |  1  | x+1 |    | 011111 =    1F   | plux  |
    // | 11 |  1  |  1  |  0  |  1  |  1  |  1  | y+1 |    | 110111 =    37   | pluy  |
    // | 12 |  0  |  0  |  1  |  1  |  1  |  0  | x-1 |    | 001110 =    0E   | subx  |
    // | 13 |  1  |  1  |  0  |  0  |  1  |  0  | y-1 |    | 110010 =    32   | suby  |
    // | 14 |  0  |  0  |  0  |  0  |  1  |  0  | x+y |    | 000010 =    02   | add   |
    // | 15 |  0  |  1  |  0  |  0  |  1  |  1  | x-y |    | 010011 =    13   | sub   |
    // | 16 |  0  |  0  |  0  |  1  |  1  |  1  | y-x |    | 000111 =    07   | rsub  |
    // | 17 |  0  |  0  |  0  |  0  |  0  |  0  | x&y |    | 000000 =    00   | and   |
    // | 18 |  0  |  1  |  0  |  1  |  0  |  1  | y|y |    | 010101 =    15   | or    |
    // |----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+

    // 4-bit Example: (row 6 out= !x)
    // x: 1 1 0 0
    // y: 1 0 1 1
    //
    // Following pre-setting:
    // x: 1 1 0 0
    // y: 1 1 1 1
    //
    // Computing and post-setting:
    // x&y: 1 1 0 0
    // !(x&y): 0 0 1 1
}
