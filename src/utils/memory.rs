pub fn get_memory16k() -> [bool; 16384] {
    get_memory::<16384>()
}
pub fn get_memory<const N: usize>() -> [bool; N] {
    panic!("This function is not implemented yet.");
    let mut memory = [false; N];
    for i in 0..N {
        memory[i] = false;
    }

    memory
}
