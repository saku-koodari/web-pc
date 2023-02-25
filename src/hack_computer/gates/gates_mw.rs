// Create multi-way gates:
//   - Or8Way
//   - Mux4Way16
//   - Mux8Way16
//   - DMux4Way
//   - DMux8Way

use crate::hack_computer::gates::gates_b16::demux16;
use crate::hack_computer::gates::gates_b16::mux16;

use super::gates_b1::or;

pub fn or8way(a: [bool; 8]) -> bool {
    or(
        a[0],
        or(a[1], or(a[2], or(a[3], or(a[4], or(a[5], or(a[6], a[7])))))),
    )
}

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
    mux16(mux16(a, b, s[0]), mux16(c, d, s[0]), s[1])
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

pub fn demux4way(
    input: [bool; 16],
    s: [bool; 2],
) -> ([bool; 16], [bool; 16], [bool; 16], [bool; 16]) {
    let (ab, cd) = demux16(input, s[1]);
    let (a, b) = demux16(ab, s[0]);
    let (c, d) = demux16(cd, s[0]);

    (a, b, c, d)
}

pub fn dmux8way(
    input: [bool; 16],
    s: [bool; 3],
) -> (
    [bool; 16],
    [bool; 16],
    [bool; 16],
    [bool; 16],
    [bool; 16],
    [bool; 16],
    [bool; 16],
    [bool; 16],
) {
    let (abcd, efgh) = demux16(input, s[2]);
    let (ab, cd) = demux16(abcd, s[1]);
    let (ef, gh) = demux16(efgh, s[1]);

    let (a, b) = demux16(ab, s[0]);
    let (c, d) = demux16(cd, s[0]);
    let (e, f) = demux16(ef, s[0]);
    let (g, h) = demux16(gh, s[0]);

    (a, b, c, d, e, f, g, h)
}

// TODO: Add tests for the new gates
