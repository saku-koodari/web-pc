use std::fmt;

pub struct ConvertResult {
    pub int_value: i16,
    pub hex_string: String,
    pub bin_array: [bool; 16],
    pub bin_array_as_string: String,
}

impl fmt::Display for ConvertResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "i16: {}, hex: {}, bin: {:?}",
            self.int_value, self.hex_string, self.bin_array_as_string
        )
    }
}

pub fn from_string(str: &str) -> Result<ConvertResult, String> {
    let int_value_res = str.parse::<i16>();
    if int_value_res.is_err() {
        return Err(format!("Cannot convert '{}' into i16 value", str));
    }

    let int_value = int_value_res.unwrap();
    let hex_string = format!("{:04X}", int_value);
    let bin_array = i16_to_b16(int_value);
    let bin_array_as_string = format!("{:016b}", int_value);
    Ok(ConvertResult {
        int_value,
        hex_string,
        bin_array,
        bin_array_as_string,
    })
}

pub fn from_b16(b16: [bool; 16]) -> ConvertResult {
    let int_value = b16_to_i16(b16);
    let hex_string = format!("{:04X}", int_value);
    let bin_array_as_string = format!("{:016b}", int_value);
    ConvertResult {
        int_value,
        hex_string,
        bin_array: b16,
        bin_array_as_string,
    }
}

/// Converts a 16-bit array to an integer, LSB first.
pub fn b16_to_i16(b16: [bool; 16]) -> i16 {
    let mut res: i16 = 0;
    for (i, &bit) in b16.iter().enumerate() {
        if bit {
            let _ = res += 1 << i;
        }
    }
    res
}

/// Converts an 32-bit integer to a 16-bit array, LSB first.
pub fn i16_to_b16(n: i16) -> [bool; 16] {
    let mut b16 = [false; 16];
    let mut quotient = n;
    let mut index = 0;
    while quotient > 0 {
        b16[index] = quotient % 2 == 1;
        quotient /= 2;
        index += 1;
    }
    b16
}

mod tests {
    #[test]
    fn test_internal_int_to_b16() {
        use crate::utils::{
            constants::i16_consts::{
                B16_0, B16_MINUS_1, B16_MINUS_2, B16_MINUS_21845, B16_MINUS_32768, B16_PLUS_1,
                B16_PLUS_2, B16_PLUS_21845, B16_PLUS_32767,
            },
            convert::i16_to_b16,
        };

        assert_eq!(i16_to_b16(0), B16_0);
        assert_eq!(i16_to_b16(1), B16_PLUS_1);
        assert_eq!(i16_to_b16(-1), B16_MINUS_1);
        assert_eq!(i16_to_b16(2), B16_PLUS_2);
        assert_eq!(i16_to_b16(-2), B16_MINUS_2);
        assert_eq!(i16_to_b16(21845), B16_PLUS_21845);
        assert_eq!(i16_to_b16(-21845), B16_MINUS_21845);
        assert_eq!(i16_to_b16(32767), B16_PLUS_32767);
        assert_eq!(i16_to_b16(-32768), B16_MINUS_32768);
    }

    #[test]
    fn test_internal_b16_to_int() {
        use crate::utils::{
            constants::i16_consts::{
                B16_0, B16_MINUS_1, B16_MINUS_2, B16_MINUS_21845, B16_MINUS_32768, B16_PLUS_1,
                B16_PLUS_2, B16_PLUS_21845, B16_PLUS_32767,
            },
            convert::b16_to_i16,
        };

        assert_eq!(b16_to_i16(B16_0), 0);
        assert_eq!(b16_to_i16(B16_PLUS_1), 1);
        assert_eq!(b16_to_i16(B16_PLUS_2), 2);
        assert_eq!(b16_to_i16(B16_PLUS_21845), 21845);
        assert_eq!(b16_to_i16(B16_PLUS_32767), 32767);
        assert_eq!(b16_to_i16(B16_MINUS_32768), -32768);
        assert_eq!(b16_to_i16(B16_MINUS_21845), 21845);
        assert_eq!(b16_to_i16(B16_MINUS_2), 2);
        assert_eq!(b16_to_i16(B16_MINUS_1), 1);
    }

    // test cases for from_string
    struct TestCasesFromString {
        string_input: String,

        int_value: i16,
        hex_string: String,
        bin_array: [bool; 16],
        bin_array_as_string: String,
        is_err: bool,
    }

    #[test]
    fn test_from_string() {
        use crate::utils::{
            constants::i16_consts::{B16_0, B16_PLUS_1},
            convert::from_string,
        };

        let test_cases: Vec<TestCasesFromString> = vec![
            TestCasesFromString {
                string_input: "0".to_string(),
                int_value: 0,
                hex_string: "0000".to_string(),
                bin_array: B16_0,
                bin_array_as_string: "0000000000000000".to_string(),
                is_err: false,
            },
            TestCasesFromString {
                string_input: "1".to_string(),
                int_value: 1,
                hex_string: "0001".to_string(),
                bin_array: B16_PLUS_1,
                bin_array_as_string: "0000000000000001".to_string(),
                is_err: false,
            },
        ];

        for test_case in test_cases {
            let res = from_string(&test_case.string_input);
            assert_eq!(res.is_err(), test_case.is_err);

            if res.is_err() {
                continue;
            }

            let res = res.unwrap();
            assert_eq!(res.int_value, test_case.int_value);
            assert_eq!(res.hex_string, test_case.hex_string);
            assert_eq!(res.bin_array, test_case.bin_array);
            assert_eq!(res.bin_array_as_string, test_case.bin_array_as_string);
        }
    }

    #[test]
    fn test_from_b16() {
        use crate::utils::{
            constants::i16_consts::{B16_0, B16_PLUS_1},
            convert::from_b16,
        };

        let res = from_b16(B16_0);
        assert_eq!(res.int_value, 0);
        assert_eq!(res.hex_string, "0000");
        assert_eq!(res.bin_array, B16_0);
        assert_eq!(res.bin_array_as_string, "0000000000000000");

        let res = from_b16(B16_PLUS_1);
        assert_eq!(res.int_value, 1);
        assert_eq!(res.hex_string, "0001");
        assert_eq!(res.bin_array, B16_PLUS_1);
        assert_eq!(res.bin_array_as_string, "0000000000000001");
    }
}
