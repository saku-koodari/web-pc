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
    // unit tests for ALU (arithmatic logic unit)

    use std::collections::HashMap;

    use crate::utils::convert::{from_i16, from_string_binary};

    fn i16_to_b16(i: i16) -> [bool; 16] {
        from_i16(i).unwrap().as_array_b16
    }

    fn bin_str_to_b16(s: String) -> [bool; 16] {
        from_string_binary(s).unwrap().as_array_b16
    }




    fn b(i: i16) -> bool {
        i == 1
    }



    // tests are not fully convered, because there are too many cases to test
    // that's why there's an UI to test the ALU
    // TODO: Make test cases for all opcodes, but using static x and y values (then out can be also static)
    #[test]
    fn test_alu_test_cases_1() {
        use crate::pc::chips::alu::alu;


        let input_x = bin_str_to_b16(String::from("1110101110000110")); // -5242
        let input_y = bin_str_to_b16(String::from("0001100001101101")); // 6253

        let negation_input_x = bin_str_to_b16(String::from("0001010001111001"));
        let negation_input_x = bin_str_to_b16(String::from("1110011110010010"));

        struct TestCase {
            arrange_x: [bool; 16],
            arrange_y: [bool; 16],
            arrange_zx: bool,
            arrange_nx: bool,
            arrange_zy: bool,
            arrange_ny: bool,
            arrange_f: bool,
            arrange_no: bool,

            assert_out: [bool; 16],
            assert_zr: bool,
            assert_ng: bool,
        }

        let test_cases = vec![
            TestCase {
                arrange_x: i16_to_b16(30),
                arrange_y: i16_to_b16(20),

                arrange_zx: false,
                arrange_nx: false,
                arrange_zy: false,
                arrange_ny: true,
                arrange_f: true,
                arrange_no: true,

                assert_out: i16_to_b16(-10),
                assert_zr: false,
                assert_ng: true,
            },
            TestCase {
                arrange_x: input_x,
                arrange_y: input_y,

                // row 17: x&y
                arrange_zx: false,
                arrange_nx: false,
                arrange_zy: false,
                arrange_ny: false,
                arrange_f: false,
                arrange_no: false,

                assert_out: bin_str_to_b16(String::from("0000100000000100")),
                assert_zr: false,
                assert_ng: false,
            },
        ];

        // loop test cases
        for test_case in test_cases {
            let actual_resul = alu(
                test_case.arrange_x,
                test_case.arrange_y,
                test_case.arrange_zx,
                test_case.arrange_nx,
                test_case.arrange_zy,
                test_case.arrange_ny,
                test_case.arrange_f,
                test_case.arrange_no,
            );

            assert_eq!(actual_resul.0, test_case.assert_out);
            assert_eq!(actual_resul.1, test_case.assert_zr);
            assert_eq!(actual_resul.2, test_case.assert_ng);
        }
    }
}
