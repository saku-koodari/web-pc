// -- 
// -- DO NOT EDIT THIS FILE
// -- This file is generated by scripts/opcodes.js --
// --   at 2023-02-24T06:09:55.878Z --
// 
// -- If you want to edit this, go to {repo}/scripts - folder, 
// --   and run `npm run start`.
// -- The new file will be generated in {repo}/scripts/generated - folder
// -- The code is generated from opcodes.handlebars with codes.csv
// --

use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    Zero,
    One,
    MinusOne,
    X,
    Y,
    NegX,
    NegY,
    MinusX,
    MinusY,
    XPlusOne,
    YPlusOne,
    XMinusOne,
    YMinusOne,
    XPlusY,
    XMinusY,
    YMinusX,
    XAndY,
    YOrY,
}

#[derive(Debug, Clone)]
pub struct AluControlBits {
    pub zx: bool,
    pub nx: bool,
    pub zy: bool,
    pub ny: bool,
    pub f: bool,
    pub no: bool,
    pub int: String,
    pub hex: String,
    pub bin: String,
    pub name: String,
}

impl fmt::Display for AluControlBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}, int: {}, hex: {}, bin: {}, 
            bytes: (zx:{}, nx:{}, zy:{}, ny:{}, f:{}, no: {})",
            self.name,
            self.int,
            self.hex,
            self.bin,
            self.zx,
            self.nx,
            self.zy,
            self.ny,
            self.f,
            self.no
        )
    }
}

pub fn get_opcodes() -> HashMap<Opcode, AluControlBits> {
    let mut opcodes = HashMap::new();

    opcodes.insert(
        Opcode::Zero,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("Zero"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::One,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("One"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::MinusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("MinusOne"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::X,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("X"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::Y,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("Y"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::NegX,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("NegX"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::NegY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("NegY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::MinusX,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("MinusX"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::MinusY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("MinusY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::XPlusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("XPlusOne"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::YPlusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("YPlusOne"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::XMinusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("XMinusOne"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::YMinusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("YMinusOne"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::XPlusY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("XPlusY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::XMinusY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("XMinusY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::YMinusX,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("YMinusX"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::XAndY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("XAndY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::YOrY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            name: String::from("YOrY"),
            int:  String::from("7"),
            hex:  String::from("0x13"),
            bin:  String::from("111"),
        },
    );

    opcodes
}
