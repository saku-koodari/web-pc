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

#[derive(Debug)]
pub struct AluControlBits {
    zx: i16,
    nx: i16,
    zy: i16,
    ny: i16,
    f: i16,
    no: i16,
    int: String,
    hex: String,
    bin: String,
}

pub fn get_opcodes() -> HashMap<Opcode, AluControlBits> {
    let mut opcodes = HashMap::new();

    opcodes.insert(
        Opcode::Zero,
        AluControlBits {
            zx: 1,
            nx: 0,
            zy: 1,
            ny: 0,
            f: 1,
            no: 0,
            int: String::from("4"),
            hex: String::from("0x10"),
            bin: String::from("100"),
        },
    );
    opcodes.insert(
        Opcode::One,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 1,
            ny: 1,
            f: 1,
            no: 1,
            int: String::from("8"),
            hex: String::from("0x20"),
            bin: String::from("1000"),
        },
    );
    opcodes.insert(
        Opcode::MinusOne,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 1,
            ny: 0,
            f: 1,
            no: 0,
            int: String::from("7"),
            hex: String::from("0x13"),
            bin: String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::X,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 1,
            ny: 1,
            f: 0,
            no: 0,
            int: String::from("6"),
            hex: String::from("0x12"),
            bin: String::from("110"),
        },
    );
    opcodes.insert(
        Opcode::Y,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 0,
            ny: 0,
            f: 0,
            no: 0,
            int: String::from("7"),
            hex: String::from("0x13"),
            bin: String::from("111"),
        },
    );
    opcodes.insert(
        Opcode::NegX,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 1,
            ny: 1,
            f: 0,
            no: 1,
            int: String::from("9"),
            hex: String::from("0x21"),
            bin: String::from("1001"),
        },
    );
    opcodes.insert(
        Opcode::NegY,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 0,
            ny: 0,
            f: 0,
            no: 1,
            int: String::from("10"),
            hex: String::from("0x22"),
            bin: String::from("1010"),
        },
    );
    opcodes.insert(
        Opcode::MinusX,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 1,
            ny: 1,
            f: 1,
            no: 1,
            int: String::from("12"),
            hex: String::from("0x30"),
            bin: String::from("1100"),
        },
    );
    opcodes.insert(
        Opcode::MinusY,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 0,
            ny: 0,
            f: 1,
            no: 1,
            int: String::from("13"),
            hex: String::from("0x31"),
            bin: String::from("1101"),
        },
    );
    opcodes.insert(
        Opcode::XPlusOne,
        AluControlBits {
            zx: 0,
            nx: 1,
            zy: 1,
            ny: 1,
            f: 1,
            no: 1,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::YPlusOne,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 0,
            ny: 1,
            f: 1,
            no: 1,
            int: String::from("16"),
            hex: String::from("0x100"),
            bin: String::from("10000"),
        },
    );
    opcodes.insert(
        Opcode::XMinusOne,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 1,
            ny: 1,
            f: 1,
            no: 0,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::YMinusOne,
        AluControlBits {
            zx: 1,
            nx: 1,
            zy: 0,
            ny: 0,
            f: 1,
            no: 0,
            int: String::from("16"),
            hex: String::from("0x100"),
            bin: String::from("10000"),
        },
    );
    opcodes.insert(
        Opcode::XPlusY,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 0,
            ny: 0,
            f: 1,
            no: 0,
            int: String::from("15"),
            hex: String::from("0x33"),
            bin: String::from("1111"),
        },
    );
    opcodes.insert(
        Opcode::XMinusY,
        AluControlBits {
            zx: 0,
            nx: 1,
            zy: 0,
            ny: 0,
            f: 1,
            no: 1,
            int: String::from("18"),
            hex: String::from("0x102"),
            bin: String::from("10010"),
        },
    );
    opcodes.insert(
        Opcode::YMinusX,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 0,
            ny: 1,
            f: 1,
            no: 1,
            int: String::from("19"),
            hex: String::from("0x103"),
            bin: String::from("10011"),
        },
    );
    opcodes.insert(
        Opcode::XAndY,
        AluControlBits {
            zx: 0,
            nx: 0,
            zy: 0,
            ny: 0,
            f: 0,
            no: 0,
            int: String::from("17"),
            hex: String::from("0x101"),
            bin: String::from("10001"),
        },
    );
    opcodes.insert(
        Opcode::XOrY,
        AluControlBits {
            zx: 0,
            nx: 1,
            zy: 0,
            ny: 1,
            f: 0,
            no: 1,
            int: String::from("21"),
            hex: String::from("0x111"),
            bin: String::from("10101"),
        },
    );

    opcodes
}
