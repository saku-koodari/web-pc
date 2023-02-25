use crate::hack_computer::{gates::gates_mw::{dmux8way, mux8way16}, chips::registers::{register_16bit_unsafe, register_1bit_unsafe}};

pub fn ram8(
    input: [bool;16], 
    load: bool, 
    address: [bool; 3]) -> [bool; 16] {

 
    //  DMux8Way(in=load, sel=address, a=l0, b=l1, c=l2, d=l3, e=l4, f=l5, g=l6, h=l7);
    let dmux_out = dmux8way(load, address);

    let out_reg0 = register_16bit_unsafe(input, dmux_out.0);
    let out_reg1 = register_16bit_unsafe(input, dmux_out.1);
    let out_reg2 = register_16bit_unsafe(input, dmux_out.2);
    let out_reg3 = register_16bit_unsafe(input, dmux_out.3);
    let out_reg4 = register_16bit_unsafe(input, dmux_out.4);
    let out_reg5 = register_16bit_unsafe(input, dmux_out.5);
    let out_reg6 = register_16bit_unsafe(input, dmux_out.6);
    let out_reg7 = register_16bit_unsafe(input, dmux_out.7);

    mux8way16(
        out_reg0,
        out_reg1,
        out_reg2,
        out_reg3,
        out_reg4,
        out_reg5,
        out_reg6,
        out_reg7,
        address
    )
}