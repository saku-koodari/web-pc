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

#[derive(Debug)]
pub struct UnsignedConvertResult<const N: usize> {
    pub as_array_b_nsize: [bool; N],
    pub as_u64: u64,
    pub as_string_hex: String,
    pub as_string_bin: String,
}

pub fn from_string_unsigned_integer<const N: usize>(
    int_str: String,
) -> Result<UnsignedConvertResult<N>, String> {
    let int_conv_res = u64::from_str_radix(&int_str, 10);
    if int_conv_res.is_err() {
        let error = int_conv_res.err().unwrap();
        return Err(format!(
            "Cannot convert '{}' into i64 value because {}",
            int_str, error
        ));
    }

    let as_u64 = int_conv_res.unwrap();
    Ok(from_u64::<N>(as_u64))
}

pub fn from_u64<const N: usize>(int: u64) -> UnsignedConvertResult<N> {
    let as_array_b_nsize = u64_to_b_nsize(int);
    let as_string_bin = b_nsize_to_byte_string(as_array_b_nsize);
    let as_string_hex = format!("0x{:0width$X}", int, width = N / 4);

    UnsignedConvertResult {
        as_array_b_nsize,
        as_u64: int,
        as_string_hex,
        as_string_bin,
    }
}

pub fn u64_to_b_nsize<const N: usize>(int: u64) -> [bool; N] {
    let mut bool_array = [false; N];
    for i in 0..N {
        bool_array[i] = (int >> i) & 1 == 1;
    }
    bool_array
}

pub fn b_nsize_to_byte_string<const N: usize>(b_nsize: [bool; N]) -> String {
    let mut result = String::new();
    for i in (0..N).rev() {
        result.push(if b_nsize[i] { '1' } else { '0' });
    }
    result
}

mod test {
    #[test]
    fn test_from_string_unsigned_integer() {
        struct TestCase {
            input: String,
            expected_u64: u64,
            expected_string_hex: String,
            expected_string_bin: String,
        }

        let test_cases = vec![
            TestCase {
                input: String::from("0"),
                expected_u64: 0,
                expected_string_hex: String::from("0x000"),
                expected_string_bin: String::from("00000000000000"),
            },
            TestCase {
                input: String::from("16383"),
                expected_u64: 16383,
                expected_string_hex: String::from("0x3FFF"),
                expected_string_bin: String::from("11111111111111"),
            },
            TestCase {
                input: String::from("2058"),
                expected_u64: 2058,
                expected_string_hex: String::from("0x80A"),
                expected_string_bin: String::from("00100000001010"),
            },
        ];

        for test_case in test_cases {
            let result = super::from_string_unsigned_integer::<14>(test_case.input);

            assert!(result.is_ok());
            let actual = result.unwrap();

            assert_eq!(
                actual.as_u64, test_case.expected_u64,
                "Expected {} but it was actual {}",
                test_case.expected_u64, actual.as_u64
            );
            assert_eq!(
                actual.as_string_hex, test_case.expected_string_hex,
                "Expected {} but it was actual {}",
                test_case.expected_string_hex, actual.as_string_hex
            );
            assert_eq!(
                actual.as_string_bin, test_case.expected_string_bin,
                "Expected {} but it was actual {}",
                test_case.expected_string_bin, actual.as_string_bin
            );
        }
    }
}
