// use std::fmt;

// pub struct OLD_ConvertResult {
//     pub int_value: i16,
//     pub hex_string: String,
//     pub bin_array: [bool; 16],
//     pub bin_array_as_string: String,
// }

// impl fmt::Display for OLD_ConvertResult {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "i16: {}, hex: {}, bin: {:?}",
//             self.int_value, self.hex_string, self.bin_array_as_string
//         )
//     }
// }

// pub fn OLD_from_string(str: &str) -> Result<OLD_ConvertResult, String> {
//     let int_value_res = str.parse::<i16>();
//     if int_value_res.is_err() {
//         return Err(format!("Cannot convert '{}' into i16 value", str));
//     }

//     let int_value = int_value_res.unwrap();
//     Ok(from_i16(int_value))
// }

// /*
// 0|000   0  0
// 0|001   1  1
// 0|010   2  2
// 0|011   3  3
// 0|100   4  4
// 0|101   5  5
// 0|110   6  6
// 0|111   7  7
// 1|000 - 8  8
// 1|001 - 7  9
// 1|010 - 6 10
// 1|011 - 5 11
// 1|100 - 4 12
// 1|101 - 3 13
// 1|110 - 2 14
// 1|111 - 1 15
//  */
// pub fn OLD_from_b16(b16: [bool; 16]) -> OLD_ConvertResult {
//     let int_val = OLD_b16_to_i16(b16);
//     from_i16(int_val)
// }

// pub fn from_i16(int_value: i16) -> OLD_ConvertResult {
//     let bin_array_as_string = OLD_b16_to_string(OLD_i16_to_b16(int_value));
//     // print!("binary string (ouputted) {bin_array_as_string}\n");
//     //println!("Hex: {:04X}\n", int_value);

//     OLD_ConvertResult {
//         int_value,
//         hex_string: format!("{:04X}", int_value),
//         bin_array: OLD_str_to_b16(&bin_array_as_string),
//         bin_array_as_string,
//     }
// }

// pub fn OLD_b16_to_i16(b16: [bool; 16]) -> i16 {
//     let mut i16_val: i16 = 0;
//     for i in 0..15 {
//         if b16[i] {
//             i16_val |= 1 << i;
//         }
//     }

//     // print!("test\n");
//     if b16[15] {
//         i16_val |= -32768; // Set signed bit if the 15th bit is true
//     }
//     i16_val
// }

// fn OLD_i16_to_b16(num: i16) -> [bool; 16] {
//     let mut bits: [bool; 16] = [false; 16];
//     let mut n = num.abs() as u16;

//     for i in 0..15 {
//         bits[i] = n & 0x01 == 0x01;
//         n >>= 1;
//     }

//     if num < 0 {
//         bits[15] = true;
//         for i in 0..15 {
//             bits[i] = !bits[i];
//         }
//         let mut carry = true;
//         for i in 0..15 {
//             bits[i] ^= carry;
//             carry = bits[i] && carry;
//         }
//     }

//     bits
// }

// /// Converts a 16-bit array into a string.
// pub fn OLD_b16_to_string(b16: [bool; 16]) -> String {
//     let mut result = String::new();
//     for i in 0..16 {
//         if b16[i] {
//             result.push('1');
//         } else {
//             result.push('0');
//         }
//     }
//     result
// }

// /// Converts a string representation of a binary number to a 16-bit array.
// pub fn OLD_str_to_b16(str: &str) -> [bool; 16] {
//     let mut result: [bool; 16] = [false; 16];
//     for (i, c) in str.chars().rev().enumerate() {
//         if c == '1' {
//             result[i] = true;
//         }
//     }
//     result
// }
