use super::gates_b1::{and, mux, nand, nor, not, or, xnor, xor};

/// 16-bit AND Gate
pub fn and16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

/// 16-bit  OR Gate
pub fn or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

/// 16-bit  NOT Gate
pub fn not16(a: [bool; 16]) -> [bool; 16] {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

/// 16-bit  XOR Gate
pub fn xor16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        xor(a[0], b[0]),
        xor(a[1], b[1]),
        xor(a[2], b[2]),
        xor(a[3], b[3]),
        xor(a[4], b[4]),
        xor(a[5], b[5]),
        xor(a[6], b[6]),
        xor(a[7], b[7]),
        xor(a[8], b[8]),
        xor(a[9], b[9]),
        xor(a[10], b[10]),
        xor(a[11], b[11]),
        xor(a[12], b[12]),
        xor(a[13], b[13]),
        xor(a[14], b[14]),
        xor(a[15], b[15]),
    ]
}

/// 16-bit  NAND Gate
pub fn nand16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        nand(a[0], b[0]),
        nand(a[1], b[1]),
        nand(a[2], b[2]),
        nand(a[3], b[3]),
        nand(a[4], b[4]),
        nand(a[5], b[5]),
        nand(a[6], b[6]),
        nand(a[7], b[7]),
        nand(a[8], b[8]),
        nand(a[9], b[9]),
        nand(a[10], b[10]),
        nand(a[11], b[11]),
        nand(a[12], b[12]),
        nand(a[13], b[13]),
        nand(a[14], b[14]),
        nand(a[15], b[15]),
    ]
}

/// 16-bit  NOR Gate
pub fn nor16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        nor(a[0], b[0]),
        nor(a[1], b[1]),
        nor(a[2], b[2]),
        nor(a[3], b[3]),
        nor(a[4], b[4]),
        nor(a[5], b[5]),
        nor(a[6], b[6]),
        nor(a[7], b[7]),
        nor(a[8], b[8]),
        nor(a[9], b[9]),
        nor(a[10], b[10]),
        nor(a[11], b[11]),
        nor(a[12], b[12]),
        nor(a[13], b[13]),
        nor(a[14], b[14]),
        nor(a[15], b[15]),
    ]
}

/// 16-bit  XNOR Gate
pub fn xnor16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    [
        xnor(a[0], b[0]),
        xnor(a[1], b[1]),
        xnor(a[2], b[2]),
        xnor(a[3], b[3]),
        xnor(a[4], b[4]),
        xnor(a[5], b[5]),
        xnor(a[6], b[6]),
        xnor(a[7], b[7]),
        xnor(a[8], b[8]),
        xnor(a[9], b[9]),
        xnor(a[10], b[10]),
        xnor(a[11], b[11]),
        xnor(a[12], b[12]),
        xnor(a[13], b[13]),
        xnor(a[14], b[14]),
        xnor(a[15], b[15]),
    ]
}

/// 16-bit  MUX Gate
pub fn mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

/// 16-bit  DMUX
pub fn demux16(a: [bool; 16], sel: bool) -> ([bool; 16], [bool; 16]) {
    (
        [
            and(a[0], not(sel)),
            and(a[1], not(sel)),
            and(a[2], not(sel)),
            and(a[3], not(sel)),
            and(a[4], not(sel)),
            and(a[5], not(sel)),
            and(a[6], not(sel)),
            and(a[7], not(sel)),
            and(a[8], not(sel)),
            and(a[9], not(sel)),
            and(a[10], not(sel)),
            and(a[11], not(sel)),
            and(a[12], not(sel)),
            and(a[13], not(sel)),
            and(a[14], not(sel)),
            and(a[15], not(sel)),
        ],
        [
            and(a[0], sel),
            and(a[1], sel),
            and(a[2], sel),
            and(a[3], sel),
            and(a[4], sel),
            and(a[5], sel),
            and(a[6], sel),
            and(a[7], sel),
            and(a[8], sel),
            and(a[9], sel),
            and(a[10], sel),
            and(a[11], sel),
            and(a[12], sel),
            and(a[13], sel),
            and(a[14], sel),
            and(a[15], sel),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const B16_TRUE: [bool; 16] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];
    const B16_FALSE: [bool; 16] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    #[test]
    fn test_and() {
        assert_eq!(and16(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(and16(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(and16(B16_FALSE, B16_TRUE), B16_FALSE);
        assert_eq!(and16(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_or() {
        assert_eq!(or16(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(or16(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(or16(B16_FALSE, B16_TRUE), B16_TRUE);
        assert_eq!(or16(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_not() {
        assert_eq!(not16(B16_TRUE), B16_FALSE);
        assert_eq!(not16(B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor16(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(xor16(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(xor16(B16_FALSE, B16_TRUE), B16_TRUE);
        assert_eq!(xor16(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_nand() {
        assert_eq!(nand16(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(nand16(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(nand16(B16_FALSE, B16_TRUE), B16_TRUE);
        assert_eq!(nand16(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_nor() {
        assert_eq!(nor16(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(nor16(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(nor16(B16_FALSE, B16_TRUE), B16_FALSE);
        assert_eq!(nor16(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(xnor16(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(xnor16(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(xnor16(B16_FALSE, B16_TRUE), B16_FALSE);
        assert_eq!(xnor16(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux16(B16_TRUE, B16_FALSE, false), B16_TRUE);
        assert_eq!(mux16(B16_TRUE, B16_FALSE, true), B16_FALSE);
        assert_eq!(mux16(B16_FALSE, B16_TRUE, false), B16_FALSE);
        assert_eq!(mux16(B16_FALSE, B16_TRUE, true), B16_TRUE);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(demux16(B16_TRUE, false), (B16_TRUE, B16_FALSE));
        assert_eq!(demux16(B16_TRUE, true), (B16_FALSE, B16_TRUE));
        assert_eq!(demux16(B16_FALSE, false), (B16_FALSE, B16_FALSE));
        assert_eq!(demux16(B16_FALSE, true), (B16_FALSE, B16_FALSE));
    }
}
