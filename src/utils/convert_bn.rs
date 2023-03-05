use std::fmt;

#[derive(Debug)]
pub struct ConvertNResult<const N: usize> {
    pub as_bin_array: [bool; N],
    pub as_integer: i16,
    pub as_usize: usize,
    pub as_string_hex: String,
    pub as_string_bin: String,
}

impl fmt::Display for ConvertNResult<16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "i16: {}, hex: {}, bin: {:?}, bytes: {:?}",
            self.as_integer, self.as_string_hex, self.as_string_bin, self.as_bin_array
        )
    }
}

pub fn from_bool_array<const N: usize>(b_arr: [bool; N]) -> Result<ConvertNResult<N>, String> {
    let mut as_string_bin = bn_size_to_byte_string(b_arr);
    if as_string_bin.is_empty() {
        // TODO: support N != 16
        if N != 16 {
            return Err(format!(
                "Other than 16 bits with default values are not supported yet."
            ));
        }

        as_string_bin = String::from("0000000000000000");
    }

    let int32_conversion = i32::from_str_radix(&as_string_bin, 2);
    if int32_conversion.is_err() {
        let error = int32_conversion.err().unwrap();
        return Err(format!(
            "Cannot convert '{}' into i16 value because {}",
            as_string_bin, error
        ));
    }

    let as_int32 = int32_conversion.unwrap();
    let as_integer = as_int32 as i16;

    Ok(ConvertNResult {
        as_bin_array: b_arr,
        as_integer,
        as_usize: as_integer as usize,
        as_string_hex: format!("0x{:04X}", as_integer),
        as_string_bin,
    })
}

pub fn from_i16<const N: usize>(integer: i16) -> Result<ConvertNResult<N>, String> {
    let as_string_bin = format!("{:016b}", integer);
    let as_bin_array = byte_string_to_b_arr(as_string_bin.to_owned());

    Ok(ConvertNResult {
        as_bin_array,
        as_integer: integer,
        as_usize: integer as usize,
        as_string_hex: format!("0x{:04X}", integer),
        as_string_bin,
    })
}

pub fn from_string_binary<const N: usize>(str: String) -> Result<ConvertNResult<N>, String> {
    if str.len() != 16 {
        return Err(format!("Cannot convert '{str}' binary into i16 value because it is not 16 bits long. It is {len} bits long.", str=str, len=str.len()));
    }

    if !str.chars().all(|c| c == '0' || c == '1') {
        return Err(format!("Cannot convert '{str}' binary into i16 value because it contains non-binary characters.", str=str));
    }

    // 1110 1011 1000 0110
    let b16 = byte_string_to_b_arr(str);
    from_bool_array(b16)
}

pub fn from_string_integer<const N: usize>(str: String) -> Result<ConvertNResult<N>, String> {
    let int_value_res = str.parse::<i16>();
    if int_value_res.is_err() {
        return Err(format!("Cannot convert '{str}' into i16 value"));
    }

    from_i16(int_value_res.unwrap())
}

// these will be tested via test_from_* - tests.
// not the most unit-testy way,
// but modularitizing the methods would be overkill.

fn bn_size_to_byte_string<const N: usize>(b_arr: [bool; N]) -> String {
    let mut result = String::new();
    for i in (0..N).rev() {
        result.push(if b_arr[i] { '1' } else { '0' });
    }
    result
}

fn byte_string_to_b_arr<const N: usize>(byte_string: String) -> [bool; N] {
    let mut result = [false; N];
    for (i, s) in byte_string.chars().rev().enumerate() {
        result[i] = s == '1';
    }
    result
}

mod tests {

    // TODO: test the error cases

    #[test]
    fn test_from_bool_array() {
        use crate::utils::{
            constants::i16_consts::{
                B16_0, B16_MINUS_1, B16_MINUS_2, B16_MINUS_21846, B16_MINUS_32768, B16_PLUS_1,
                B16_PLUS_2, B16_PLUS_21845, B16_PLUS_32767,
            },
            convert_bn::from_bool_array,
        };
        assert_eq!(from_bool_array(B16_0).unwrap().as_integer, 0);
        assert_eq!(from_bool_array(B16_PLUS_1).unwrap().as_integer, 1);
        assert_eq!(from_bool_array(B16_PLUS_2).unwrap().as_integer, 2);
        assert_eq!(from_bool_array(B16_PLUS_21845).unwrap().as_integer, 21845);
        assert_eq!(from_bool_array(B16_PLUS_32767).unwrap().as_integer, 32767);
        assert_eq!(from_bool_array(B16_MINUS_32768).unwrap().as_integer, -32768);
        assert_eq!(from_bool_array(B16_MINUS_21846).unwrap().as_integer, -21846);
        assert_eq!(from_bool_array(B16_MINUS_2).unwrap().as_integer, -2);
        assert_eq!(from_bool_array(B16_MINUS_1).unwrap().as_integer, -1);
    }

    #[test]
    fn test_from_i16() {
        use crate::utils::{
            constants::i16_consts::{
                B16_0, B16_MINUS_1, B16_MINUS_2, B16_MINUS_21846, B16_MINUS_32768, B16_PLUS_1,
                B16_PLUS_2, B16_PLUS_21845, B16_PLUS_32767,
            },
            convert_bn::from_i16,
        };

        // The test needs to be against the binary constants
        // so that it tests the conversion from i16 to b16.
        assert_eq!(from_i16(0).unwrap().as_bin_array, B16_0);
        assert_eq!(from_i16(1).unwrap().as_bin_array, B16_PLUS_1);
        assert_eq!(from_i16(2).unwrap().as_bin_array, B16_PLUS_2);
        assert_eq!(from_i16(21845).unwrap().as_bin_array, B16_PLUS_21845);
        assert_eq!(from_i16(32767).unwrap().as_bin_array, B16_PLUS_32767);
        assert_eq!(from_i16(-32768).unwrap().as_bin_array, B16_MINUS_32768);
        assert_eq!(from_i16(-21846).unwrap().as_bin_array, B16_MINUS_21846);
        assert_eq!(from_i16(-2).unwrap().as_bin_array, B16_MINUS_2);
        assert_eq!(from_i16(-1).unwrap().as_bin_array, B16_MINUS_1);
    }

    #[test]
    fn test_from_string_integer() {
        use crate::utils::{
            constants::i16_consts::{
                B16_0, B16_MINUS_1, B16_MINUS_2, B16_MINUS_21846, B16_MINUS_32768, B16_PLUS_1,
                B16_PLUS_2, B16_PLUS_21845, B16_PLUS_32767,
            },
            convert_bn::from_string_integer,
        };

        // Arrange
        let str_zero = String::from("0");
        let str_plus_one = String::from("1");
        let str_plus_two = String::from("2");
        let str_plus_21845 = String::from("21845");
        let str_plus_32767 = String::from("32767");
        let str_minus_32768 = String::from("-32768");
        let str_minus_21846 = String::from("-21846");
        let str_minus_two = String::from("-2");
        let str_minus_one = String::from("-1");

        // Act
        let res_zero = from_string_integer(str_zero).unwrap();
        let res_plus_one = from_string_integer(str_plus_one).unwrap();
        let res_plus_two = from_string_integer(str_plus_two).unwrap();
        let res_plus_21845 = from_string_integer(str_plus_21845).unwrap();
        let res_plus_32767 = from_string_integer(str_plus_32767).unwrap();
        let res_minus_32768 = from_string_integer(str_minus_32768).unwrap();
        let res_minus_21846 = from_string_integer(str_minus_21846).unwrap();
        let res_minus_two = from_string_integer(str_minus_two).unwrap();
        let res_minus_one = from_string_integer(str_minus_one).unwrap();

        // Assert
        assert_eq!(res_zero.as_bin_array, B16_0);
        assert_eq!(res_plus_one.as_bin_array, B16_PLUS_1);
        assert_eq!(res_plus_two.as_bin_array, B16_PLUS_2);
        assert_eq!(res_plus_21845.as_bin_array, B16_PLUS_21845);
        assert_eq!(res_plus_32767.as_bin_array, B16_PLUS_32767);
        assert_eq!(res_minus_32768.as_bin_array, B16_MINUS_32768);
        assert_eq!(res_minus_21846.as_bin_array, B16_MINUS_21846);
        assert_eq!(res_minus_two.as_bin_array, B16_MINUS_2);
        assert_eq!(res_minus_one.as_bin_array, B16_MINUS_1);
    }
}
