// the changes are isolated from other parts of the code
// and therefore duplicate code.

pub fn i16_to_bn_arr<const N: usize>(num: i16) -> [bool; N] {
    let mut result = [false; N];
    for i in 0..N {
        result[(N - 1) - i] = (num >> i) & 1 == 1;
    }
    result
}

pub fn bn_to_i16<const N: usize>(arr: [bool; N]) -> i16 {
    let mut result = 0;
    for i in 0..N {
        result |= (arr[(N - 1) - i] as i16) << i;
    }
    result
}

pub fn get_bit_from_i16(val: i16, bit_index: usize) -> bool {
    ((val >> bit_index) & 1) == 1
}

pub fn set_bit_from_i16(val: i16, bit_index: usize, bit_value: bool) -> i16 {
    if bit_value {
        val | (1 << bit_index)
    } else {
        val & !(1 << bit_index)
    }
}
