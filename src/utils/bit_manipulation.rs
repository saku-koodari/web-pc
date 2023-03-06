pub fn get_bit_from_i16(value: i16, index: usize) -> bool {
    ((value >> index) & 1) == 1
}

pub fn set_bit_from_u16(value: i16, index: usize, new_value: bool) -> i16 {
    if new_value {
        value | (1 << index)
    } else {
        value & !(1 << index)
    }
}
