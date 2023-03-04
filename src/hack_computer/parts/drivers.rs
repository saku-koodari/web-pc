use crate::hack_computer::{
    gates::{gates_b1::not, gates_b16::mux16},
    ram::ram::Ram4k,
    registers::register_16bit::Register16Bit,
};

// 8192 words
pub struct Screen {
    ram1: Ram4k,
    ram2: Ram4k,
}

impl Screen {
    pub fn power_on() -> Self {
        Self {
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
        let out1 = self.ram1.ram4k(input, not(address[12]), address, clock);
        let out2 = self.ram1.ram4k(input, address[12], address, clock);

        //mux
        mux16(out1, out2, address[12])
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
