/// 16-bit array of all false.
pub const B16_FALSE: [bool; 16] = [
    false, false, false, false, // row 0
    false, false, false, false, // row 1
    false, false, false, false, // row 2
    false, false, false, false, // row 3
];

/// 16-bit array of all false.
pub const B16_TRUE: [bool; 16] = [
    true, true, true, true, // row 0
    true, true, true, true, // row 1
    true, true, true, true, // row 2
    true, true, true, true, // row 3
];

/// 16-bit LSB binary value of integer 0.
pub const B16_LSB_0: [bool; 16] = [
    false, false, false, false, // row 0
    false, false, false, false, // row 1
    false, false, false, false, // row 2
    false, false, false, false, // row 3
];

/// 16-bit LSB binary value of integer 1.
pub const B16_LSB_1: [bool; 16] = [
    true, false, false, false, // row 0
    false, false, false, false, // row 1
    false, false, false, false, // row 2
    false, false, false, false, // row 3
];

/// 16-bit LSB binary value of integer 2.
pub const B16_LSB_2: [bool; 16] = [
    false, true, false, false, // row 0
    false, false, false, false, // row 1
    false, false, false, false, // row 2
    false, false, false, false, // row 3
];

/// 16-bit LSB binary value of integer 3.
pub const B16_LSB_3: [bool; 16] = [
    true, true, false, false, // row 0
    false, false, false, false, // row 1
    false, false, false, false, // row 2
    false, false, false, false, // row 3
];

/// 16-bit LSB binary value of integer 43690.
pub const B16_LSB_43690: [bool; 16] = [
    false, true, false, true, // row 0
    false, true, false, true, // row 1
    false, true, false, true, // row 2
    false, true, false, true, // row 3
];

/// 16-bit LSB binary value of integer 65534.
pub const B16_LSB_65534: [bool; 16] = [
    false, true, true, true, // row 0
    true, true, true, true, // row 1
    true, true, true, true, // row 2
    true, true, true, true, // row 3
];

/// 16-bit LSB binary value of integer 65535.
pub const B16_LSB_65535: [bool; 16] = [
    true, true, true, true, // row 0
    true, true, true, true, // row 1
    true, true, true, true, // row 2
    true, true, true, true, // row 3
];
