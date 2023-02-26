use crate::hack_computer::gates::gates_b1::{and, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (xor(a, b), and(a, b))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, or(carry1, carry2))
}

/// Adds two 16-bit binary numbers.
/// Does not handle negative numbers,
/// because the adder is using one's complement repsrentation.
pub fn adder_b16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let (sum00, c01) = half_adder(a[0], b[0]);
    let (sum01, c02) = full_adder(a[1], b[1], c01);
    let (sum02, c03) = full_adder(a[2], b[2], c02);
    let (sum03, c04) = full_adder(a[3], b[3], c03);
    let (sum04, c05) = full_adder(a[4], b[4], c04);
    let (sum05, c06) = full_adder(a[5], b[5], c05);
    let (sum06, c07) = full_adder(a[6], b[6], c06);
    let (sum07, c08) = full_adder(a[7], b[7], c07);
    let (sum08, c09) = full_adder(a[8], b[8], c08);
    let (sum09, c10) = full_adder(a[9], b[9], c09);
    let (sum10, c11) = full_adder(a[10], b[10], c10);
    let (sum11, c12) = full_adder(a[11], b[11], c11);
    let (sum12, c13) = full_adder(a[12], b[12], c12);
    let (sum13, c14) = full_adder(a[13], b[13], c13);
    let (sum14, c15) = full_adder(a[14], b[14], c14);
    let (sum15, _) = full_adder(a[15], b[15], c15);

    [
        sum00, sum01, sum02, sum03, sum04, sum05, sum06, sum07, sum08, sum09, sum10, sum11, sum12,
        sum13, sum14, sum15,
    ]
}

pub fn inc16(input: [bool; 16]) -> [bool; 16] {
    let (sum00, c01) = half_adder(input[0], true);
    let (sum01, c02) = half_adder(input[1], c01);
    let (sum02, c03) = half_adder(input[2], c02);
    let (sum03, c04) = half_adder(input[3], c03);
    let (sum04, c05) = half_adder(input[4], c04);
    let (sum05, c06) = half_adder(input[5], c05);
    let (sum06, c07) = half_adder(input[6], c06);
    let (sum07, c08) = half_adder(input[7], c07);
    let (sum08, c09) = half_adder(input[8], c08);
    let (sum09, c10) = half_adder(input[9], c09);
    let (sum10, c11) = half_adder(input[10], c10);
    let (sum11, c12) = half_adder(input[11], c11);
    let (sum12, c13) = half_adder(input[12], c12);
    let (sum13, c14) = half_adder(input[13], c13);
    let (sum14, c15) = half_adder(input[14], c14);
    let (sum15, _) = half_adder(input[15], c15);

    [
        sum00, sum01, sum02, sum03, sum04, sum05, sum06, sum07, sum08, sum09, sum10, sum11, sum12,
        sum13, sum14, sum15,
    ]
}

mod tests {
    use crate::utils::convert_16b::ConvertResult;

    #[test]
    fn test_half_adder() {
        use crate::hack_computer::chips::adder::half_adder;

        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    #[test]
    fn test_full_adder() {
        use crate::hack_computer::chips::adder::full_adder;
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }

    #[test]
    fn test_adder_b16() {
        use crate::{
            hack_computer::chips::adder::adder_b16,
            utils::convert_16b::{from_b16, from_i16},
        };

        struct TestCase {
            input_a: Result<ConvertResult, String>,
            input_b: Result<ConvertResult, String>,
            output: Result<ConvertResult, String>,
            name: String,
        }

        // arrange
        let test_cases = vec![
            TestCase {
                input_a: from_i16(0),
                input_b: from_i16(0),
                output: from_i16(0),
                name: String::from("test 1: 0 + 0 = 0"),
            },
            TestCase {
                input_a: from_i16(1),
                input_b: from_i16(2),
                output: from_i16(3),
                name: String::from("test 4: 1 + 2 = 3"),
            },
            TestCase {
                input_a: from_i16(5),
                input_b: from_i16(-3),
                output: from_i16(2),
                name: String::from("test 5: 5 - 3 = 2"),
            },
            TestCase {
                input_a: from_i16(0),
                input_b: from_i16(-100),
                output: from_i16(-100),
                name: String::from("test 6: 0 - 100 = -100"),
            },
        ];

        for case in test_cases {
            print!("\ntesting {n}...\n", n = case.name);

            let input_a = case.input_a.unwrap();
            let input_b = case.input_b.unwrap();

            println!("a: {input_a}");
            println!("b: {input_b}");

            // act
            let res = adder_b16(input_a.as_array_b16, input_b.as_array_b16);

            // debug
            // print!(
            //     "expected: {output}. actual: {res}.\n",
            //     output = case.output,
            //     res = from_b16(res)
            // );

            // assert
            let conv_res = from_b16(res).unwrap();
            assert_eq!(case.output.unwrap().as_integer, conv_res.as_integer);

            // debug
            print!("Test passed.\n");
        }
    }

    #[test]
    fn test_inc16() {
        use crate::{
            hack_computer::chips::adder::inc16,
            utils::convert_16b::{from_b16, from_i16},
        };

        struct TestCase {
            input: Result<ConvertResult, String>,
            output: Result<ConvertResult, String>,
            name: String,
        }

        // arrange
        let test_cases = vec![
            TestCase {
                input: from_i16(0),
                output: from_i16(1),
                name: String::from("test 1: 0 + 1 = 1"),
            },
            TestCase {
                input: from_i16(1),
                output: from_i16(2),
                name: String::from("test 2: 1 + 1 = 2"),
            },
            TestCase {
                input: from_i16(5),
                output: from_i16(6),
                name: String::from("test 4: 5 + 1 = 6"),
            },
            TestCase {
                input: from_i16(-1),
                output: from_i16(0),
                name: String::from("test 5: -1 + 1 = 0"),
            },
            TestCase {
                input: from_i16(-5),
                output: from_i16(-4),
                name: String::from("test 6: -5 + 1 = -4"),
            },
        ];

        for case in test_cases {
            print!("\ntesting {n}...\n", n = case.name);

            let input = case.input.unwrap();

            println!("input: {input}");

            // act
            let res = inc16(input.as_array_b16);

            let expected = case.output.unwrap().as_integer;
            let actual = from_b16(res).unwrap().as_integer;

            assert_eq!(expected, actual);

            // debug
            // print!(
            //     "expected: {output}. actual: {res}.\n",
        }
    }

    // TODO: test adder_rca_b16 with overflow
    // TODO: test inc16
}
