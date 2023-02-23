use std::collections::HashMap;

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
    XOrY,
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
}

pub fn get_opcodes() -> HashMap<Opcode, AluControlBits> {
    let mut opcodes = HashMap::new();

    opcodes.insert(
        Opcode::Zero,
        AluControlBits {
            zx: true,
            nx: false,
            zy: true,
            ny: false,
            f: true,
            no: false,
            int: String::from("4"),
            hex: String::from("0x10"),
            bin: String::from("100"),
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
            int: String::from("8"),
            hex: String::from("0x20"),
            bin: String::from("1000"),
        },
    );
    opcodes.insert(
        Opcode::MinusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: true,
            ny: false,
            f: true,
            no: false,
            int: String::from("7"),
            hex: String::from("0x13"),
            bin: String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::X,
        AluControlBits {
            zx: false,
            nx: false,
            zy: true,
            ny: true,
            f: false,
            no: false,
            int: String::from("6"),
            hex: String::from("0x12"),
            bin: String::from("110"),
        },
    );
    opcodes.insert(
        Opcode::Y,
        AluControlBits {
            zx: true,
            nx: true,
            zy: false,
            ny: false,
            f: false,
            no: false,
            int: String::from("7"),
            hex: String::from("0x13"),
            bin: String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::NegX,
        AluControlBits {
            zx: false,
            nx: false,
            zy: true,
            ny: true,
            f: false,
            no: true,
            int: String::from("9"),
            hex: String::from("0x21"),
            bin: String::from("1001"),
        },
    );
    opcodes.insert(
        Opcode::NegY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: false,
            ny: false,
            f: false,
            no: true,
            int: String::from("10"),
            hex: String::from("0x22"),
            bin: String::from("1010"),
        },
    );
    opcodes.insert(
        Opcode::MinusX,
        AluControlBits {
            zx: false,
            nx: false,
            zy: true,
            ny: true,
            f: true,
            no: true,
            int: String::from("12"),
            hex: String::from("0x30"),
            bin: String::from("1100"),
        },
    );
    opcodes.insert(
        Opcode::MinusY,
        AluControlBits {
            zx: true,
            nx: true,
            zy: false,
            ny: false,
            f: true,
            no: true,
            int: String::from("13"),
            hex: String::from("0x31"),
            bin: String::from("1101"),
        },
    );
    opcodes.insert(
        Opcode::XPlusOne,
        AluControlBits {
            zx: false,
            nx: true,
            zy: true,
            ny: true,
            f: true,
            no: true,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::YPlusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: false,
            ny: true,
            f: true,
            no: true,
            int: String::from("16"),
            hex: String::from("0x100"),
            bin: String::from("10000"),
        },
    );
    opcodes.insert(
        Opcode::XMinusOne,
        AluControlBits {
            zx: false,
            nx: false,
            zy: true,
            ny: true,
            f: true,
            no: false,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::YMinusOne,
        AluControlBits {
            zx: true,
            nx: true,
            zy: false,
            ny: false,
            f: true,
            no: false,
            int: String::from("16"),
            hex: String::from("0x100"),
            bin: String::from("10000"),
        },
    );
    opcodes.insert(
        Opcode::XPlusY,
        AluControlBits {
            zx: false,
            nx: false,
            zy: false,
            ny: false,
            f: true,
            no: false,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::XMinusY,
        AluControlBits {
            zx: false,
            nx: true,
            zy: false,
            ny: false,
            f: true,
            no: true,
            int: String::from("18"),
            hex: String::from("0x102"),
            bin: String::from("10010"),
        },
    );
    opcodes.insert(
        Opcode::YMinusX,
        AluControlBits {
            zx: false,
            nx: false,
            zy: false,
            ny: true,
            f: true,
            no: true,
            int: String::from("19"),
            hex: String::from("0x103"),
            bin: String::from("10011"),
        },
    );
    opcodes.insert(
        Opcode::XAndY,
        AluControlBits {
            zx: false,
            nx: false,
            zy: false,
            ny: false,
            f: false,
            no: false,
            int: String::from("17"),
            hex: String::from("0x101"),
            bin: String::from("10001"),
        },
    );
    opcodes.insert(
        Opcode::XOrY,
        AluControlBits {
            zx: false,
            nx: true,
            zy: false,
            ny: true,
            f: false,
            no: true,
            int: String::from("21"),
            hex: String::from("0x111"),
            bin: String::from("10101"),
        },
    );

    opcodes
}
