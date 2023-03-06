use crate::hack_computer::{
    gates::{gates_b1::not, gates_b16::mux16},
    ram::ram4k::Ram4k,
};

// 8192 words
pub struct Screen {
    // data: [Register16Bit; 8192],
    ram1: Ram4k,
    ram2: Ram4k,
}

impl Screen {
    // fn init_data() -> [Register16BitEmulated; 8192] {
    //     let mut data: Vec<Register16BitEmulated> = Vec::new();
    //     for i in 0..8192 {
    //         data.push(Register16BitEmulated::power_on());
    //     }

    //     data.try_into().unwrap()
    // }

    pub fn power_on() -> Self {
        Self {
            // data: Self::init_data(),
            ram1: Ram4k::power_on(),
            ram2: Ram4k::power_on(),
        }
    }

    // https://youtu.be/1_TEVI0YpI0?t=565
    //
    // index 0: -> row 0, column 0
    // index 1: -> row 0, column 1
    // index  31: -> row 0, column 31
    // index  32: -> row 1, column 0
    // index  33: -> row 1, column 1
    // index  63: -> row 2, column 0
    // index  8159: -> row 255, column 0
    // index  8160: -> row 255, column 1
    // index  8191: -> row 255, column 31
    //
    // to turn pixel (row,col) on/off
    // w = Screen[row*32 + col/16]
    //
    // in the hack memory context:
    // w = Memory[16384 + row*32 + col/16]
    //
    // set col%16 bit of w to 1 or to 0

    // TODO: Test
    /// RAM 4K
    /// Rwister count: 4096
    pub fn screen(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 12],
        clock: bool,
    ) -> [bool; 16] {
        //demux
        let out1 = self.ram1.ram4k(input, not(address[11]), address, clock);
        let out2 = self.ram2.ram4k(input, address[11], address, clock);

        //mux
        mux16(out1, out2, address[11])
    }
}
