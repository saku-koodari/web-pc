/// Constant values to represent 16-bit i16 values in the virtual PC.
pub mod b16_consts {
    /// 16-bit value:
    /// - Integer 0
    /// - Binary: 0000 0000 0000 0000
    /// - Hexadecimal: 0x0000
    pub const B16_FALSE: [bool; 16] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    /// 16-bit value:
    /// - Integer -1.
    /// - Binary: 1111 1111 1111 1111.
    /// - Hexadecimal: 0xFFFF.
    pub const B16_TRUE: [bool; 16] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];
}

/// Constant values to represent 16-bit i16 values in the virtual PC.
pub mod i16_consts {
    /// 16-bit value:
    /// - Integer 0
    /// - Binary: 0000 0000 0000 0000
    /// - Hexadecimal: 0x0000
    pub const B16_0: [bool; 16] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    /// 16-bit value:
    /// - Integer 1
    /// - Binary: 0000 0000 0000 0001
    /// - Hexadecimal: 0x0001
    pub const B16_PLUS_1: [bool; 16] = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    /// 16-bit value:
    /// - Integer 2
    /// - Binary: 0000 0000 0000 0010
    /// - Hexadecimal: 0x0002
    pub const B16_PLUS_2: [bool; 16] = [
        false, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    /// FIXED
    /// 16-bit value:
    ///  - Integer 21845.
    ///  - Binary: 1010 1010 1010 1010
    ///  - Hexadecimal: 0x5555.
    pub const B16_PLUS_21845: [bool; 16] = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ];

    /// 16-bit value:
    /// - Integer 32767 (i16 max value).
    /// - Binary: 0111 1111 1111 1111
    /// - Hexadecimal: 0x7FFF.
    pub const B16_PLUS_32767: [bool; 16] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        false,
    ];

    /// 16-bit value:
    /// - Integer -32768 (i16 min value).
    /// - Binary: 1000 0000 0000 0000.
    /// - Hexadecimal: 0x8000.
    pub const B16_MINUS_32768: [bool; 16] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true,
    ];

    /// 16-bit value:
    /// - Integer -21845.
    /// - Binary: 1010 1010 1010 1010
    /// - Hexadecimal: 0xAAAA
    pub const B16_MINUS_21845: [bool; 16] = [
        false, true, false, true, false, true, false, true, false, true, false, true, false, true,
        false, true,
    ];

    /// 16-bit value:
    /// - Integer -2.
    /// - Binary: 1111 1111 1111 1110.
    /// - Hexadecimal: 0xFFFE.
    pub const B16_MINUS_2: [bool; 16] = [
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];

    /// 16-bit value:
    /// - Integer -1.
    /// - Binary: 1111 1111 1111 1111.
    /// - Hexadecimal: 0xFFFF.
    pub const B16_MINUS_1: [bool; 16] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];
}

// This file has one test to show how bits are represented in the virtual PC.
// Bits are LSB and signed. Sign bit is always last element in array.
mod test {
    use super::*;

    /// prints the bit array in printable format
    /// the algorithm is the test, therefore this logic is not in the main code.
    /// if you get unsure about the bit representation, you can rely on this test.
    fn b2str(b16: [bool; 16]) -> String {
        let mut s = String::new();
        for i in (0..16).rev() {
            s.push(if b16[i] { '1' } else { '0' });
        }

        s
    }

    #[test]
    fn test_b16_consts() {
        let zero = "0000000000000000";
        let plus_1 = "0000000000000001";
        let plus_2 = "0000000000000010";
        let plus_21845 = "0101010101010101";
        let plus_32767 = "0111111111111111";
        let minus_32768 = "1000000000000000";
        let minus_21845 = "1010101010101010";
        let minus_2 = "1111111111111110";
        let minus_1 = "1111111111111111";

        // for actual testing
        assert_eq!(b2str(i16_consts::B16_0), zero);
        assert_eq!(b2str(i16_consts::B16_PLUS_1), plus_1);
        assert_eq!(b2str(i16_consts::B16_PLUS_2), plus_2);
        assert_eq!(b2str(i16_consts::B16_PLUS_21845), plus_21845);
        assert_eq!(b2str(i16_consts::B16_PLUS_32767), plus_32767);
        assert_eq!(b2str(i16_consts::B16_MINUS_32768), minus_32768);
        assert_eq!(b2str(i16_consts::B16_MINUS_21845), minus_21845);
        assert_eq!(b2str(i16_consts::B16_MINUS_2), minus_2);
        assert_eq!(b2str(i16_consts::B16_MINUS_1), minus_1);
    }
}
