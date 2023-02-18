use crate::pc::gates::gates_b16::mux16;

// our ALU can't do multiplication or division
// they will be implemented on the software level.
// However, that will be a trade-off between speed and having more "hardware".
pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (
    [bool; 16], // out
    bool,       // zr
    bool,       // ng
) {
    // Manipulates the x and y inputs as follows:
    // if (zx == 1)  sets x = 0       // 16-bit true constant
    let out = mux16(x, [false; 16], zx);

    // if (nx == 1)  sets x = !x      // bitwise NOT
    // if (zy == 1)  sets y = 0       // 16-bit true constant
    // if (ny == 1)  sets y = !y      // bitwise NOT
    // if (f == 1)   sets out = x + y // int. 2's s-complement addition
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
    // +----+-----+-----+-----+-----+-----+-----+-----+
    // | ## | zx  |  x  | zy  | ny  |  f  | no  | out |
    // +----+-----+-----+-----+-----+-----+-----+-----+
    // | 01 |  1  |  0  |  1  |  0  |  1  |  0  |  0  |
    // | 02 |  1  |  1  |  1  |  1  |  1  |  1  |  1  |
    // | 03 |  1  |  1  |  1  |  0  |  1  |  0  | -1  |
    // | 04 |  0  |  0  |  1  |  1  |  0  |  0  |  x  |
    // | 05 |  1  |  1  |  0  |  0  |  0  |  0  |  y  |
    // | 06 |  0  |  0  |  1  |  1  |  0  |  1  | !x  |
    // | 07 |  1  |  1  |  0  |  0  |  0  |  1  | !y  |
    // | 08 |  0  |  0  |  1  |  1  |  1  |  1  | -x  |
    // | 09 |  1  |  1  |  0  |  0  |  1  |  1  | -y  |
    // | 10 |  0  |  1  |  1  |  1  |  1  |  1  | x+1 |
    // | 11 |  1  |  1  |  0  |  1  |  1  |  1  | y+1 |
    // | 12 |  0  |  0  |  1  |  1  |  1  |  0  | x-1 |
    // | 13 |  1  |  1  |  0  |  0  |  1  |  0  | y-1 |
    // | 14 |  0  |  0  |  0  |  0  |  1  |  0  | x+y |
    // | 15 |  0  |  1  |  0  |  0  |  1  |  1  | x-y |
    // | 16 |  0  |  0  |  0  |  1  |  1  |  1  | y-x |
    // | 17 |  0  |  0  |  0  |  0  |  0  |  0  | x&y |
    // | 18 |  0  |  1  |  0  |  1  |  0  |  1  | y|y |
    // |-+-----+-----+-----+-----+-----+-----+-----+

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
