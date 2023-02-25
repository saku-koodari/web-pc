
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

    let set = and(data, store);
    let reset = and(not(data), store);

    unsafe {
        NOR_OUT1 = nor(NOR_OUT1, reset);
        NOR_OUT2 = nor(NOR_OUT2, set);

        return NOR_OUT2;
    }
}

mod test {
    use super::*;

    #[test]
    fn test_d_latch() {
        panic!("Not implemented");
    }
}