use crate::hack_computer::{
    chips::registers::{register_16bit_unsafe, register_1bit_unsafe},
    gates::gates_mw::{dmux8way, mux8way16, mux4way16},
};

pub fn ram8(input: [bool; 16], load: bool, address: [bool; 3]) -> [bool; 16] {
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
        out_reg0, out_reg1, out_reg2, out_reg3, out_reg4, out_reg5, out_reg6, out_reg7, address,
    )
}

pub fn ram64(input: [bool; 16], load: bool, address: [bool; 6]) -> [bool; 16] {
    let dmux_out = dmux8way(load, [address[3], address[4], address[5]]);

    let a = ram8(input, dmux_out.0, [address[0], address[1], address[2]]);
    let b = ram8(input, dmux_out.1, [address[0], address[1], address[2]]);
    let c = ram8(input, dmux_out.2, [address[0], address[1], address[2]]);
    let d = ram8(input, dmux_out.3, [address[0], address[1], address[2]]);
    let e = ram8(input, dmux_out.4, [address[0], address[1], address[2]]);
    let f = ram8(input, dmux_out.5, [address[0], address[1], address[2]]);
    let g = ram8(input, dmux_out.6, [address[0], address[1], address[2]]);
    let h = ram8(input, dmux_out.7, [address[0], address[1], address[2]]);

    mux8way16(a, b, c, d, e, f, g, h, [address[3], address[4], address[5]])
}

pub fn ram512(input: [bool; 16], load: bool, address: [bool; 9]) -> [bool; 16] {
    let dmux_out = dmux8way(load, [address[6], address[7], address[8]]);

    let a = ram64(input, dmux_out.0, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let b = ram64(input, dmux_out.1, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let c = ram64(input, dmux_out.2, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let d = ram64(input, dmux_out.3, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let e = ram64(input, dmux_out.4, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let f = ram64(input, dmux_out.5, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let g = ram64(input, dmux_out.6, [address[0], address[1], address[2], address[3], address[4], address[5]]);
    let h = ram64(input, dmux_out.7, [address[0], address[1], address[2], address[3], address[4], address[5]]);

    mux8way16(a, b, c, d, e, f, g, h, [address[6], address[7], address[8]])
}

pub fn ram4k(input: [bool; 16], load: bool, address: [bool; 12]) -> [bool; 16] {
    let dmux_out = dmux8way(load, [address[9], address[10], address[11]]);

    let a = ram512(input, dmux_out.0, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let b = ram512(input, dmux_out.1, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let c = ram512(input, dmux_out.2, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let d = ram512(input, dmux_out.3, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let e = ram512(input, dmux_out.4, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let f = ram512(input, dmux_out.5, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let g = ram512(input, dmux_out.6, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);
    let h = ram512(input, dmux_out.7, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]);

    mux8way16(a, b, c, d, e, f, g, h, [address[9], address[10], address[11]])
}

pub fn ram16k(input: [bool; 16], load: bool, address: [bool; 14]) -> [bool; 16] {
    let dmux_out = dmux8way(load, [address[11], address[12], address[13]]);

    let a = ram4k(input, dmux_out.0, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11]]);
    let b = ram4k(input, dmux_out.1, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11]]);
    let c = ram4k(input, dmux_out.2, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11]]);
    let d = ram4k(input, dmux_out.3, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11]]);

    mux4way16(a, b, c, d, [address[12], address[13]])
}