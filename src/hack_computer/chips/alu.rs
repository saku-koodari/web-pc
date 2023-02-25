use crate::{
    hack_computer::{
        chips::adder::adder_b16,
        gates::{
            gates_b1::{mux, not, or},
            gates_b16::{and16, demux16, mux16, not16},
            gates_mw::{or16way, or8way},
        },
    },
    utils::{convert_16b, opcodes::AluControlBits},
};

use super::adder::inc16;

// the truth table for the ALU
// +----+-----------------------------------+-----+
// |    |            control bits           | out |
// +----+-----------------------------------+-----+
// |    | preset-x  | preset-y  | sel |post | out |
// +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-----------+
// | ## | zx  | nx  | zy  | ny  |  f  | no  | out | => |  6-bit = opcodes | instruct. | test-output
// +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-----------+------------------+-- +---+
// | 01 |  1  |  0  |  1  |  0  |  1  |  0  |  0  |    | 101010 =    2A   | Zero      | out[16] 0000000000000000 zr 1 ng    0
// | 02 |  1  |  1  |  1  |  1  |  1  |  1  |  1  |    | 111111 =    3F   | One       | out[16] 0000000000000001 zr 0 ng    0
// | 03 |  1  |  1  |  1  |  0  |  1  |  0  | -1  |    | 111010 =    3E   | MinusOne  | out[16]	1111111111111111 zr	0 ng	1
// | 04 |  0  |  0  |  1  |  1  |  0  |  0  |  x  |    | 001100 =    0C   | X         | out[16]	1110101110000110 zr	0 ng	1
// | 05 |  1  |  1  |  0  |  0  |  0  |  0  |  y  |    | 110000 =    30   | Y         | out[16]	0001100001101101 zr 0 ng	0
// | 06 |  0  |  0  |  1  |  1  |  0  |  1  | !x  |    | 001101 =    0D   | NegX      | out[16]	0001010001111001 zr	0 ng	0
// | 07 |  1  |  1  |  0  |  0  |  0  |  1  | !y  |    | 110001 =    31   | NegY      | out[16]	1110011110010010 zr	0 ng	1
// | 08 |  0  |  0  |  1  |  1  |  1  |  1  | -x  |    | 001111 =    0F   | MinusX    | out[16]	0001010001111010 zr	0 ng	0
// | 09 |  1  |  1  |  0  |  0  |  1  |  1  | -y  |    | 110011 =    33   | MinusY    | out[16]	1110011110010011 zr	0 ng	1
// | 10 |  0  |  1  |  1  |  1  |  1  |  1  | x+1 |    | 011111 =    1F   | XPlusOne  | out[16]	1110101110000111 zr	0 ng	1
// | 11 |  1  |  1  |  0  |  1  |  1  |  1  | y+1 |    | 110111 =    37   | YPlusOne  | out[16]	0001100001101110 zr	0 ng	0
// | 12 |  0  |  0  |  1  |  1  |  1  |  0  | x-1 |    | 001110 =    0E   | XMinusOne | out[16]	1110101110000101 zr	0 ng	1
// | 13 |  1  |  1  |  0  |  0  |  1  |  0  | y-1 |    | 110010 =    32   | YMinusOne | out[16]	0001100001101100 zr	0 ng	0
// | 14 |  0  |  0  |  0  |  0  |  1  |  0  | x+y |    | 000010 =    02   | XPlusY    | out[16]	0000001111110011 zr	0 ng	0
// | 15 |  0  |  1  |  0  |  0  |  1  |  1  | x-y |    | 010011 =    13   | XMinusY   | out[16]	1101001100011001 zr	0 ng	1
// | 16 |  0  |  0  |  0  |  1  |  1  |  1  | y-x |    | 000111 =    07   | YMinusX   | out[16]	0010110011100111 zr	0 ng	0
// | 17 |  0  |  0  |  0  |  0  |  0  |  0  | x&y |    | 000000 =    00   | XAndY     | out[16]	0000100000000100 zr	0 ng	0
// | 18 |  0  |  1  |  0  |  1  |  0  |  1  | y|y |    | 010101 =    15   | YOrY      | out[16]	1111101111101111 zr	0 ng	1
// |----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-----------+------------------+-- +---+

fn zero_negator(input: [bool; 16], zero: bool, negate: bool) -> [bool; 16] {
    let z_res = mux16(input,  [false; 16], zero);
    mux16(z_res, not16(z_res), negate)
}

fn func(input_a: [bool; 16], input_b: [bool; 16], func: bool) -> [bool; 16] {
    mux16(and16(input_a, input_b), adder_b16(input_a, input_b), func)
}

// our ALU can't do multiplication or division
// they will be implemented on the software level.
// However, that will be a trade-off between speed and having more "hardware".
pub fn alu(
    x: [bool; 16], // 16-bit input x
    y: [bool; 16], // 16-bit input y
    zx: bool,      // zero the x input?
    nx: bool,      // negate the x input?
    zy: bool,      // zero the y input?
    ny: bool,      // negate the y input?
    f: bool,       // function selector
    no: bool,      // negate the output?
) -> (
    [bool; 16], // out
    bool,       // zr: zero result
    bool,       // ng: negative result
) {
    // like with full adder, this chip has a drawback
    // you can't run gates in parallel, because the application has to excecute bits one by one
    // this might cause performance issues.
    let out_func = func(zero_negator(x, zx, nx), zero_negator(y, zy, ny), f);
    let out = mux16(out_func, not16(out_func), no);

    (
        out,
        not(or16way(input)) // zr
        input[15] // ng
    )
}

mod tests {
    use crate::utils::convert_16b::{self, from_i16, from_string_binary};

    // unit tests for ALU (arithmatic logic unit)
    fn i16_to_b16(i: i16) -> [bool; 16] {
        from_i16(i).unwrap().as_array_b16
    }

    fn bin_str_to_b16(s: String) -> [bool; 16] {
        from_string_binary(s).unwrap().as_array_b16
    }

    // tests are not fully convered, because there are too many cases to test
    // that's why there's an UI to test the ALU
    #[test]
    fn test_alu_test_cases_1() {
        use crate::hack_computer::chips::alu::alu;
        use crate::utils::{
            convert_16b::from_b16,
            opcodes::{get_opcodes, AluControlBits, Opcode},
        };

        #[derive(Debug)]
        struct AluTestCase {
            opcode: Option<AluControlBits>,

            expect_out: [bool; 16],
            expect_zr: bool, // zero result
            expect_ng: bool, // negative result
        }

        /// 1110101110000110 = -5242
        fn get_input_x() -> [bool; 16] {
            bin_str_to_b16(String::from("1110101110000110"))
        }

        /// 0001100001101101 = 6253
        fn get_input_y() -> [bool; 16] {
            bin_str_to_b16(String::from("0001100001101101"))
        }

        let opcodes = get_opcodes();

        // left: actual (code/test), right: expected (here)
        let test_cases = vec![
            AluTestCase {
                opcode: opcodes.get(&Opcode::Zero).cloned(),
                expect_out: bin_str_to_b16(String::from("0000000000000000")),
                expect_zr: true,
                expect_ng: false,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::One).cloned(),
                expect_out: bin_str_to_b16(String::from("0000000000000001")),
                expect_zr: false,
                expect_ng: false,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::MinusOne).cloned(),
                expect_out: bin_str_to_b16(String::from("1111111111111111")),
                expect_zr: false,
                expect_ng: true,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::X).cloned(),
                expect_out: get_input_x(), // -5242
                expect_zr: false,
                expect_ng: true,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::Y).cloned(),
                expect_out: get_input_y(), // 6253
                expect_zr: false,
                expect_ng: false,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::NegX).cloned(),
                expect_out: bin_str_to_b16(String::from("0001010001111001")),
                expect_zr: false,
                expect_ng: false,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::NegY).cloned(),
                expect_out: bin_str_to_b16(String::from("1110011110010010")),
                expect_zr: false,
                expect_ng: true,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::MinusX).cloned(),
                expect_out: bin_str_to_b16(String::from("0001010001111010")), // FAILS
                expect_zr: false,
                expect_ng: false,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::MinusY).cloned(),
                expect_out: bin_str_to_b16(String::from("1110011110010011")), // NOT TESTED
                expect_zr: false,
                expect_ng: true,
            },
            AluTestCase {
                opcode: opcodes.get(&Opcode::XPlusOne).cloned(),
                expect_out: bin_str_to_b16(String::from("1110101110000111")), // NOT TESTED
                expect_zr: false,
                expect_ng: true,
            },
            // YPlusOne,    // out[16]	0001100001101110 zr	0 ng	0
            AluTestCase {
                opcode: opcodes.get(&Opcode::YPlusOne).cloned(),
                expect_out: bin_str_to_b16(String::from("0001100001101110")), // NOT TESTED
                expect_zr: false,
                expect_ng: false,
            },
            // XMinusOne,   // out[16]	1110101110000101 zr	0 ng	1
            AluTestCase {
                opcode: opcodes.get(&Opcode::XMinusOne).cloned(),
                expect_out: bin_str_to_b16(String::from("1110101110000101")), // NOT TESTED
                expect_zr: false,
                expect_ng: true,
            },
            // YMinusOne,   // out[16]	0001100001101100 zr	0 ng	0
            AluTestCase {
                opcode: opcodes.get(&Opcode::YMinusOne).cloned(),
                expect_out: bin_str_to_b16(String::from("0001100001101100")), // NOT TESTED
                expect_zr: false,
                expect_ng: false,
            },
            // XPlusY,      // out[16]	0000001111110011 zr	0 ng	0
            AluTestCase {
                opcode: opcodes.get(&Opcode::XPlusY).cloned(),
                expect_out: bin_str_to_b16(String::from("0000001111110011")), // NOT TESTED
                expect_zr: false,
                expect_ng: false,
            },
            // XMinusY,     // out[16]	1101001100011001 zr	0 ng	1
            AluTestCase {
                opcode: opcodes.get(&Opcode::XMinusY).cloned(),
                expect_out: bin_str_to_b16(String::from("1101001100011001")), // NOT TESTED
                expect_zr: false,
                expect_ng: true,
            },
            // YMinusX,     // out[16]	0010110011100111 zr	0 ng	0
            AluTestCase {
                opcode: opcodes.get(&Opcode::YMinusX).cloned(),
                expect_out: bin_str_to_b16(String::from("0010110011100111")), // NOT TESTED
                expect_zr: false,
                expect_ng: false,
            },
            // XAndY,       // out[16]	0000100000000100 zr	0 ng	0
            AluTestCase {
                opcode: opcodes.get(&Opcode::XAndY).cloned(),
                expect_out: bin_str_to_b16(String::from("0000100000000100")), // NOT TESTED
                expect_zr: false,
                expect_ng: false,
            },
            // YOrY,        // out[16]	1111101111101111 zr	0 ng	1
            AluTestCase {
                opcode: opcodes.get(&Opcode::YOrY).cloned(),
                expect_out: bin_str_to_b16(String::from("1111101111101111")), // NOT TESTED
                expect_zr: false,
                expect_ng: true,
            },
        ];

        // loop test cases
        for test_case in test_cases {
            let alu_control_bits = test_case.opcode.unwrap();
            println!("TESTING: '{}'", alu_control_bits.name);

            let actual_result = alu(
                get_input_x(),
                get_input_y(),
                alu_control_bits.zx,
                alu_control_bits.nx,
                alu_control_bits.zy,
                alu_control_bits.ny,
                alu_control_bits.f,
                alu_control_bits.no,
            );

            let expected_out = from_b16(test_case.expect_out).unwrap();
            let actual_out = from_b16(actual_result.0).unwrap();

            print!("\n");
            println!("debugging after act...");
            println!("control bits: {:?}", alu_control_bits);
            println!(
                "expected   out: {}, {:?}, zr: {},  ng: {}",
                expected_out.as_integer,
                expected_out.as_string_bin,
                test_case.expect_zr,
                test_case.expect_ng
            );
            println!(
                "actual     out: {}, {:?}, zr: {},  ng: {}",
                actual_out.as_integer, actual_out.as_string_bin, actual_result.1, actual_result.2
            );

            print!("\n");
            println!("asserting...");
            assert_eq!(actual_out.as_integer, expected_out.as_integer);
            assert_eq!(actual_result.1, test_case.expect_zr, "WRONG ZERO RESULT");
            assert_eq!(
                actual_result.2, test_case.expect_ng,
                "WRONG NEGATIVE RESULT"
            );
        }
    }
}
