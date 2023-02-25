use crate::utils::opcodes::AluControlBits;

pub fn alu_opcode_from_bytes(
    zx: bool, // zero the x input?
    nx: bool, // negate the x input?
    zy: bool, // zero the y input?
    ny: bool, // negate the y input?
    f: bool,  // function selector
    no: bool, // negate the output?
) -> AluControlBits {
    let arr = [zx, nx, zy, ny, f, no];
    let bin = b_nsize_to_byte_string(arr);
    let int = u32::from_str_radix(&bin, 2).unwrap();
    let hex = format!("0x{:0width$X}", int, width = 6);

    AluControlBits {
        zx,
        nx,
        zy,
        ny,
        f,
        no,
        bin,
        int: int.to_string(),
        hex,
        name: String::from("UNKNOWN"),
    }
}

pub fn b_nsize_to_byte_string<const N: usize>(b_nsize: [bool; N]) -> String {
    let mut result = String::new();
    for i in (0..N).rev() {
        result.push(if b_nsize[i] { '1' } else { '0' });
    }
    result
}

pub fn byte_string_to_b_nsize<const N: usize>(byte_string: String) -> [bool; N] {
    let mut result = [false; N];
    for (i, s) in byte_string.chars().rev().enumerate() {
        result[i] = s == '1';
    }
    result
}
