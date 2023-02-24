// AND Gate
pub fn and(a: bool, b: bool) -> bool {
    a && b
}

// NOT Gate
pub fn not(a: bool) -> bool {
    !a
}

// OR Gate
pub fn or(a: bool, b: bool) -> bool {
    not(and(not(a), not(b)))
}

// XOR Gate
pub fn xor(a: bool, b: bool) -> bool {
    or(
        and(a, not(b)), 
        and(not(a), b)) // a ^ b
}

// NAND Gate
pub fn nand(a: bool, b: bool) -> bool {
    not(and(a, b))
}

// NOR Gate
pub fn nor(a: bool, b: bool) -> bool {
    not(or(a, b))
}

// XNOR Gate
pub fn xnor(a: bool, b: bool) -> bool {
    not(xor(a, b))
}

/// 1-bit Multiplexer. a.ka. data selector. if sel is true, return b, else return a
///
/// ## Arguments
/// * `a` - first input
/// * `b` - second input
/// * `sel` - selector
///
/// ## Returns
/// * `bool` - output
pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(
        and(a, not(sel)),
         and(b, sel))
}

// Demultiplexer
pub fn demux(input: bool, sel: bool) -> (bool, bool) {
    (and(input, not(sel)), and(input, sel))
}

// the loop is compiled at compile time
pub fn nand_gen<const N: usize>(a: [bool; N], b: [bool; N]) -> [bool; N] {
    let mut result = [false; N];
    for i in 0..N {
        result[i] = nand(a[i], b[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(true, true), true);
        assert_eq!(and(true, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(false, false), false);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(true, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(false, true), true);
        assert_eq!(or(false, false), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(false, false), false);
    }

    #[test]
    fn test_nand() {
        assert_eq!(nand(true, true), false);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(false, false), true);
    }

    #[test]
    fn test_nor() {
        assert_eq!(nor(true, true), false);
        assert_eq!(nor(true, false), false);
        assert_eq!(nor(false, true), false);
        assert_eq!(nor(false, false), true);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(xnor(true, true), true);
        assert_eq!(xnor(true, false), false);
        assert_eq!(xnor(false, true), false);
        assert_eq!(xnor(false, false), true);
    }

    #[test]
    fn test_mux() {
        /*
            input-a sel output
            a       0   a
            a       1   b
        */
        //              a     b      sel     out
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_demux() {
        /*
            input-a sel output-a output-b
            a       0   0        a
            a       1   a        0
        */
        assert_eq!(demux(false, false), (false, false));
        assert_eq!(demux(false, true), (false, false));
        assert_eq!(demux(true, false), (true, false));
        assert_eq!(demux(true, true), (false, true));
    }
}
