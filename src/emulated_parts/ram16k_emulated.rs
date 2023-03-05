use super::conv::{bn_to_i16, i16_to_bn_arr};

pub struct Ram16kEmulated {
    values: [i16; 16384],
}

impl Ram16kEmulated {
    pub fn power_on() -> Self {
        Self {
            values: [0; 16384],
        }
    }

    // +--------+------+-------+             
    // |        |      | child | total            
    // |  part  | addr | parts | parts            
    // |  name  | size | count | count | 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 |
    // +--------+------+-------+-------|-------------------------------------------------+
    // |   ram8 |  3   |   8   |     8 | xx xx xx xx xx xx xx xx xx xx xx xx xx 02 01 00 |
    // |  ram64 |  6   |   8   |    64 | xx xx xx xx xx xx xx xx xx xx 05 04 03 -- -- -- |
    // | ram512 |  8   |   8   |   512 | xx xx xx xx xx xx xx 08 07 06 -- -- -- -- -- -- |
    // |  ram4k |  12  |   8   |  4096 | xx xx xx xx 11 10 09 -- -- -- -- -- -- -- -- -- |
    // | ram16k |  14  |   4   | 16384 | xx xx 13 12 11 -- -- -- -- -- -- -- -- -- -- -- |
    // +--------+------+-------+-------|-------------------------------------------------+
    pub fn ram16k_emulated
    <const N: usize>(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; N], // 000
        // 64 -> 000
        clock: bool,
    ) -> [bool; 16] {
        
        let int_addr = bn_to_i16(address) as usize;
        if clock && load {
            self.values[int_addr] = bn_to_i16(input);
        }

        i16_to_bn_arr(self.values[int_addr])
    }

    pub fn print_ram(&self, start: usize, end: usize) {
        let mut max = end;
        if max > self.values.len() {
            max = self.values.len();
        }

        for i in start..max {
            println!("{}: {}", i, self.values[i]);
        }
    }
}