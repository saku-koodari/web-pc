use crate::hack_computer::gates::gates_b1::{and, nor, not};

/// Data flip flop. Uses unsafe code to store the NOR_OUT1 and NOR_OUT2 values.
///
/// Stores the NOR_OUT1 and NOR_OUT2 values in static variables.
/// It represents how D-Latches are implemented in hardware.
///
/// NOTE: This method is not thread safe
pub fn dff_unsafe(data: bool, store: bool) -> bool {
    // Visual reprensentation of the D-latch
    /// 1-bit
    // inputs:
    //
    // data  O---┬---+-----+ +-----+
    //           ┃           │ AND ┝ -set-------+-----+
    //           ┃   ┌-------+-----+            │ NOR ┝ -nor_out1-┐
    //           ┃   │                ┌---------+-----+           │
    //           ┃   │                │    ┌----------------------┘
    //           ┃   │                │    ┃
    //           ┃   │                └----╂----------------------┐
    //           ┃   │                     └---+-----+            |
    // store O---╂---┴-------+-----+           │ NOR ┝ -nor_out2--┴-------------- 0 out
    //           ┃           │ AND ┝ -reset----+-----+
    //           ┗━ +NOT-----+-----+
    //
    // As you can see from the visualization, these variables
    // are in order to make the cross connection between NOR gates.

    static mut NOR_OUT1: bool = false;
    static mut NOR_OUT2: bool = false;

    println!("\n\n");
    println!("data flip flop - dff(data:{}, store:{})", data, store);

    let set = and(data, store);
    println!("set: {}", set);

    let reset = and(not(data), store);
    println!("reset: {}", reset);

    unsafe {
        println!("UNSAFE START");
        NOR_OUT1 = nor(NOR_OUT1, reset);
        println!("UNSAFE NOR_OUT1: {}", NOR_OUT1);

        NOR_OUT2 = nor(NOR_OUT2, set);
        println!("RETRUN UNSAFE NOR_OUT2: {}\n\n\n", NOR_OUT2);
        return NOR_OUT2;
    }
}

mod test {
    use super::*;

    #[test]
    fn test_d_latch() {
        //panic!("Not implemented");
    }
}
