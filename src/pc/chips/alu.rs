use crate::pc::{
    chips::adder::adder_b16,
    gates::{
        gates_b1::{mux, not, or},
        gates_b16::{and16, demux16, mux16, not16},
        gates_mw::{or16way, or8way},
    },
};

// the truth table for the ALU

// +----+-----------------------------------+-----+
// |    |            control bits           | out |
// +----+-----------------------------------+-----+
// |    | preset-x  | preset-y  | sel |post | out |
// +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+
// | ## | zx  | nx  | zy  | ny  |  f  | no  | out | => |  6-bit = opcodes | instr |
// +----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+
// | 01 |  1  |  0  |  1  |  0  |  1  |  0  |  0  |    | 101010 =    2A   | reset |
// | 02 |  1  |  1  |  1  |  1  |  1  |  1  |  1  |    | 111111 =    3F   | fill  |
// | 03 |  1  |  1  |  1  |  0  |  1  |  0  | -1  |    | 111010 =    3E   | neg   |
// | 04 |  0  |  0  |  1  |  1  |  0  |  0  |  x  |    | 001100 =    0C   | x     |
// | 05 |  1  |  1  |  0  |  0  |  0  |  0  |  y  |    | 110000 =    30   | y     |
// | 06 |  0  |  0  |  1  |  1  |  0  |  1  | !x  |    | 001101 =    0D   | negx  |
// | 07 |  1  |  1  |  0  |  0  |  0  |  1  | !y  |    | 110001 =    31   | negy  |
// | 08 |  0  |  0  |  1  |  1  |  1  |  1  | -x  |    | 001111 =    0F   | xsub  |
// | 09 |  1  |  1  |  0  |  0  |  1  |  1  | -y  |    | 110011 =    33   | ysub  |
// | 10 |  0  |  1  |  1  |  1  |  1  |  1  | x+1 |    | 011111 =    1F   | plux  |
// | 11 |  1  |  1  |  0  |  1  |  1  |  1  | y+1 |    | 110111 =    37   | pluy  |
// | 12 |  0  |  0  |  1  |  1  |  1  |  0  | x-1 |    | 001110 =    0E   | subx  |
// | 13 |  1  |  1  |  0  |  0  |  1  |  0  | y-1 |    | 110010 =    32   | suby  |
// | 14 |  0  |  0  |  0  |  0  |  1  |  0  | x+y |    | 000010 =    02   | add   |
// | 15 |  0  |  1  |  0  |  0  |  1  |  1  | x-y |    | 010011 =    13   | sub   |
// | 16 |  0  |  0  |  0  |  1  |  1  |  1  | y-x |    | 000111 =    07   | rsub  |
// | 17 |  0  |  0  |  0  |  0  |  0  |  0  | x&y |    | 000000 =    00   | and   |
// | 18 |  0  |  1  |  0  |  1  |  0  |  1  | y|y |    | 010101 =    15   | or    |
// |----+-----+-----+-----+-----+-----+-----+-----+    +--------=---------+-------+

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

    // Manipulates the x and y inputs as follows:
    // if (zx == 1)  sets x = 0       // 16-bit true constant
    // if (zy == 1)  sets y = 0       // 16-bit true constant
    let w1 = mux16(x, [false; 16], zx);
    let w2 = mux16(y, [false; 16], zy);

    // if (nx == 1)  sets x = !x      // bitwise NOT
    let notw1 = not16(w1);
    let w3 = mux16(w1, notw1, nx);

    // if (ny == 1)  sets y = !y      // bitwise NOT
    let notw2 = not16(w2);
    let w4 = mux16(w2, notw2, ny);

    // if (f == 1)   sets out = x + y // int. 2's s-complement addition
    // if (f == 0)   sets out = x & y // bitwise AND
    let xandy = and16(w3, w4);
    let addxy = adder_b16(w3, w4);
    let w5 = mux16(xandy, addxy, f);

    // if (no == 1)  sets out = !out  // bitwise NOT
    // if (out < 0)  sets ng = 1      // 1-bit true constant
    let notw5 = not16(w5);
    let out = mux16(w5, notw5, no);
    let ng = out[15];

    // if (out == 0) sets zr = 1      // 1-bit true constant
    let or0to15 = or16way(out);
    let zr = not(or0to15);

    (out, zr, ng)
}

mod tests {
    use crate::utils::convert::{from_i16, from_string_binary};

    // unit tests for ALU (arithmatic logic unit)
    fn i16_to_b16(i: i16) -> [bool; 16] {
        from_i16(i).unwrap().as_array_b16
    }

    fn bin_str_to_b16(s: String) -> [bool; 16] {
        from_string_binary(s).unwrap().as_array_b16
    }

    // tests are not fully convered, because there are too many cases to test
    // that's why there's an UI to test the ALU
    // TODO: Make test cases for all opcodes, but using static x and y values (then out can be also static)
    #[test]
    fn test_alu_test_cases_1() {
        use crate::pc::chips::alu::alu;
        use crate::utils::opcodes::{get_opcodes, AluControlBits, Opcode};
        use std::collections::HashMap;

        struct TestCase {
            opcode: Option<AluControlBits>,

            assert_out: [bool; 16],
            assert_zr: bool,
            assert_ng: bool,
        }

        let negation_input_x = bin_str_to_b16(String::from("0001010001111001"));
        let negation_input_x = bin_str_to_b16(String::from("1110011110010010"));

        // type: HashMap<Opcode, AluControlBits>
        let opcodes = get_opcodes();

        let test_cases = vec![TestCase {
            opcode: opcodes.get(&Opcode::XAndY).cloned(),
            assert_out: bin_str_to_b16(String::from("0000000000000000")),
            assert_zr: true,
            assert_ng: false,
        }];

        // loop test cases
        for test_case in test_cases {
            let input_x = bin_str_to_b16(String::from("1110101110000110")); // -5242
            let input_y = bin_str_to_b16(String::from("0001100001101101")); // 6253

            let alu_control_bits = test_case.opcode.unwrap();

            let actual_result = alu(
                input_x,
                input_x,
                alu_control_bits.zx,
                alu_control_bits.nx,
                alu_control_bits.zy,
                alu_control_bits.ny,
                alu_control_bits.f,
                alu_control_bits.no,
            );

            let possibleErrorMessage = format!(
                "expected: {:?}\n \
                actual: {:?}",
                test_case.assert_out, actual_result.0
            );

            assert_eq!(actual_result.0, test_case.assert_out, possibleErrorMessage);
            assert_eq!(actual_result.1, test_case.assert_zr);
            assert_eq!(actual_result.2, test_case.assert_ng);
        }
    }
}
