use crate::utils::convert_bn::from_bool_array;

pub struct Ram16kEmulated {
    values: [i16; 16384],
}

impl Ram16kEmulated {
    pub fn power_on() -> Self {
        Self { values: [0; 16384] }
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
    pub fn ram16k_emulated<const N: usize>(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; N], // 000
        // 64 -> 000
        clock: bool,
    ) -> [bool; 16] {
        let int_addr = from_bool_array(address).ok().unwrap().as_usize;
        if clock && load {
            self.values[int_addr] = from_bool_array(input).ok().unwrap().as_integer;
        }

        crate::utils::convert_16b::from_i16(self.values[int_addr])
            .unwrap()
            .as_array_b16
    }

    pub fn get_ram(&self, start: usize, end: usize) -> Vec<(usize, i16)> {
        let mut max = end;
        if max > self.values.len() {
            max = self.values.len();
        }

        let res = Vec::new();
        for i in start..max {
            res.push((i, self.values[i]));
        }

        res
    }
}
