/// Converts a 16-bit array to an integer, LSB first.
pub fn b16_to_int_lsb(b16: [bool; 16]) -> i32 {
    let mut res: i32 = 0;
    for (i, &bit) in b16.iter().enumerate() {
        if bit {
            let _ = res += 1 << i;
        }
    }
    res
}

/// Converts an 32-bit integer to a 16-bit array, LSB first.
pub fn int_to_b16_lsb(n: i32) -> [bool; 16] {
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

/// Converts a string into LSB 16-bit array.
pub fn str_to_b16_lsb(str: &str) -> Result<[bool; 16], String> {
    let str_parse_res = str.parse::<i32>();
    if str_parse_res.is_err() {
        return Err(format!("'{}' cannot be parsed into 32-bit integer", str));
    }

    let n = str_parse_res.unwrap();

    if n.abs() >= 2i32.pow(16) {
        return Err(format!("{} is too large to fit into a 16-bit array", n));
    }

    Ok(int_to_b16_lsb(n))
}

mod tests {
    #[test]
    fn test_internal_int_to_b16() {
        use crate::utils::{
            constants::{
                B16_LSB_0, B16_LSB_1, B16_LSB_2, B16_LSB_3, B16_LSB_43690, B16_LSB_65534,
                B16_LSB_65535,
            },
            convert::int_to_b16_lsb,
        };

        assert_eq!(int_to_b16_lsb(0), B16_LSB_0);
        assert_eq!(int_to_b16_lsb(1), B16_LSB_1);
        assert_eq!(int_to_b16_lsb(2), B16_LSB_2);
        assert_eq!(int_to_b16_lsb(3), B16_LSB_3);
        assert_eq!(int_to_b16_lsb(43690), B16_LSB_43690);
        assert_eq!(int_to_b16_lsb(65534), B16_LSB_65534);
        assert_eq!(int_to_b16_lsb(65535), B16_LSB_65535);
    }

    #[test]
    fn test_internal_b16_to_int() {
        use crate::utils::{
            constants::{
                B16_LSB_0, B16_LSB_1, B16_LSB_2, B16_LSB_3, B16_LSB_43690, B16_LSB_65534,
                B16_LSB_65535,
            },
            convert::b16_to_int_lsb,
        };

        assert_eq!(b16_to_int_lsb(B16_LSB_0), 0);
        assert_eq!(b16_to_int_lsb(B16_LSB_1), 1);
        assert_eq!(b16_to_int_lsb(B16_LSB_2), 2);
        assert_eq!(b16_to_int_lsb(B16_LSB_3), 3);
        assert_eq!(b16_to_int_lsb(B16_LSB_43690), 43690);
        assert_eq!(b16_to_int_lsb(B16_LSB_65534), 65534);
        assert_eq!(b16_to_int_lsb(B16_LSB_65535), 65535);
    }
}
