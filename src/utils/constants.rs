/// Constant values to represent 16-bit i16 values in the virtual PC.
pub mod i16_consts {
    /// 16-bit value:
    /// - Integer 0
    /// - Binary: 0000 0000 0000 0000
    /// - Hexadecimal: 0x0000
    pub const B16_0: [bool; 16] = [
        false, false, false, false, // row 0
        false, false, false, false, // row 1
        false, false, false, false, // row 2
        false, false, false, false, // row 3
    ];

    /// 16-bit value:
    /// - Integer 1
    /// - Binary: 0000 0000 0000 0001
    /// - Hexadecimal: 0x0001
    pub const B16_PLUS_1: [bool; 16] = [
        true, false, false, false, // row 0
        false, false, false, false, // row 1
        false, false, false, false, // row 2
        false, false, false, false, // row 3
    ];

    /// 16-bit value:
    /// - Integer 2
    /// - Binary: 0000 0000 0000 0010
    /// - Hexadecimal: 0x0002
    pub const B16_PLUS_2: [bool; 16] = [
        false, true, false, false, // row 0
        false, false, false, false, // row 1
        false, false, false, false, // row 2
        false, false, false, false, // row 3
    ];

    /// 16-bit value:
    ///  - Integer 21845.
    ///  - Binary: 1010 1010 1010 1010
    ///  - Hexadecimal: 0x5555.
    pub const B16_PLUS_21845: [bool; 16] = [
        false, true, false, true, // row 0
        false, true, false, true, // row 1
        false, true, false, true, // row 2
        false, true, false, true, // row 3
    ];

    /// 16-bit value:
    /// - Integer 32767 (i16 max value).
    /// - Binary: 0111 1111 1111 1111
    /// - Hexadecimal: 0x7FFF.
    pub const B16_PLUS_32767: [bool; 16] = [
        true, true, true, true, // row 0
        true, true, true, true, // row 1
        true, true, true, true, // row 2
        true, true, true, false, // row 3
    ];

    /// 16-bit value:
    /// - Integer -32768 (i16 min value).
    /// - Binary: 1000 0000 0000 0000.
    /// - Hexadecimal: 0x8000.
    pub const B16_MINUS_32768: [bool; 16] = [
        false, false, false, false, // row 0
        false, false, false, false, // row 1
        false, false, false, false, // row 2
        false, false, false, true, // row 3
    ];

    /// 16-bit value:
    /// - Integer -21845.
    /// - Binary: 1010 1010 1010 1010
    /// - Hexadecimal: 0xAAAA
    pub const B16_MINUS_21845: [bool; 16] = [
        false, true, false, true, // row 0
        false, true, false, true, // row 1
        false, true, false, true, // row 2
        false, true, false, true, // row 3
    ];

    /// 16-bit value:
    /// - Integer -3.
    /// - Binary: 1111 1111 1111 1101.
    /// - Hexadecimal: 0xFFFD.
    pub const B16_MINUS_3: [bool; 16] = [
        true, false, true, true, // row 0
        true, true, true, true, // row 1
        true, true, true, true, // row 2
        true, true, true, true, // row 3
    ];

    /// 16-bit value:
    /// - Integer -2.
    /// - Binary: 1111 1111 1111 1110.
    /// - Hexadecimal: 0xFFFE.
    pub const B16_MINUS_2: [bool; 16] = [
        false, true, true, true, // row 0
        true, true, true, true, // row 1
        true, true, true, true, // row 2
        true, true, true, true, // row 3
    ];

    /// 16-bit value:
    /// - Integer -1.
    /// - Binary: 1111 1111 1111 1111.
    /// - Hexadecimal: 0xFFFF.
    pub const B16_MINUS_1: [bool; 16] = [
        true, true, true, true, // row 0
        true, true, true, true, // row 1
        true, true, true, true, // row 2
        true, true, true, true, // row 3
    ];
}
