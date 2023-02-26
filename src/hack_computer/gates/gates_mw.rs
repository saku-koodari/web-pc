// Create multi-way gates:
//   - Or8Way
//   - Mux4Way16
//   - Mux8Way16
//   - DMux4Way
//   - DMux8Way

use crate::hack_computer::gates::gates_b16::demux16;
use crate::hack_computer::gates::gates_b16::mux16;

use super::gates_b1::demux;
use super::gates_b1::or;

pub fn or8way(a: [bool; 8]) -> bool {
    or(
        a[0],
        or(a[1], or(a[2], or(a[3], or(a[4], or(a[5], or(a[6], a[7])))))),
    )
}

// Note: Extra gate comparing to original hack-computer
pub fn or16way(a: [bool; 16]) -> bool {
    // It's a bit ugly, but this you need to do, if you can't use if statement
    or(
        a[0],
        or(
            a[1],
            or(
                a[2],
                or(
                    a[3],
                    or(
                        a[4],
                        or(
                            a[5],
                            or(
                                a[6],
                                or(
                                    a[7],
                                    or(
                                        a[8],
                                        or(
                                            a[9],
                                            or(
                                                a[10],
                                                or(a[11], or(a[12], or(a[13], or(a[14], a[15])))),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

pub fn mux4way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    s: [bool; 2],
) -> [bool; 16] {
    // s: 0 0 -> a
    // s: 0 1 -> b
    // s: 1 0 -> c
    // s: 1 1 -> d

    mux16(
        mux16(a, b, s[0]), 
        mux16(c, d, s[0]), 
        s[1])
}

pub fn mux8way16(
    a: [bool; 16],
    b: [bool; 16],
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16],
    f: [bool; 16],
    g: [bool; 16],
    h: [bool; 16],
    s: [bool; 3],
) -> [bool; 16] {
    mux16(
        mux16(mux16(a, b, s[0]), mux16(c, d, s[0]), s[1]),
        mux16(mux16(e, f, s[0]), mux16(g, h, s[0]), s[1]),
        s[2],
    )
}

// NOTE: correct signature
pub fn demux4way(
    input: bool,
    s: [bool; 2],
) -> (bool, bool, bool, bool) {
    // s: 0 0 -> {in,0,0,0}
    // s: 0 1 -> {0,in,0,0}
    // s: 1 0 -> {0,0,in,0}
    // s: 1 1 -> {0,0,0,in}

    let (ab, cd) = demux(input, s[1]);
    let (a, b) = demux(ab, s[0]);
    let (c, d) = demux(cd, s[0]);

    (a, b, c, d)
}


// NOTE: correct signature
pub fn dmux8way(
    input: bool,
    s: [bool; 3],
) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    let (abcd, efgh) = demux(input, s[2]);
    let (ab, cd) = demux(abcd, s[1]);
    let (ef, gh) = demux(efgh, s[1]);

    let (a, b) = demux(ab, s[0]);
    let (c, d) = demux(cd, s[0]);
    let (e, f) = demux(ef, s[0]);
    let (g, h) = demux(gh, s[0]);

    (a, b, c, d, e, f, g, h)
}

// TODO: Add tests for the new gates
