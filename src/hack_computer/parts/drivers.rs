use crate::hack_computer::{
    gates::{gates_b1::not, gates_b16::mux16},
    ram::ram::Ram4k,
    registers::register_16bit::{Register16Bit, Register16BitEmulated},
};

// 8192 words
pub struct Screen {
    // data: [Register16Bit; 8192],
    ram1: Ram4k,
    ram2: Ram4k,
}

impl Screen {
    fn init_data() -> [Register16BitEmulated; 8192] {
        let mut data: Vec<Register16BitEmulated> = Vec::new();
        for i in 0..8192 {
            data.push(Register16BitEmulated::power_on());
        }

        data.try_into().unwrap()
    }

    pub fn power_on() -> Self {
        Self {
            // data: Self::init_data(),
            ram1: Ram4k::power_on(),
            ram2: Ram4k::power_on(),
        }
    }

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

// 1 word
pub struct Keyboard {
    data: Register16Bit,
}

impl Keyboard {
    pub fn power_on() -> Self {
        Self {
            data: Register16Bit::power_on(),
        }
    }

    /// Register 16 bit
    /// Rwister count: 1
    pub fn keyboard(&mut self, input: [bool; 16], load: bool, clock: bool) -> [bool; 16] {
        self.data.register_16bit_clocked(input, load, clock)
    }
}
