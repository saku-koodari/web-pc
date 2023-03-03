use crate::hack_computer::{
    gates::{
        gates_b1::or,
        gates_mw::{demux4way, mux4way16},
    },
    ram::ram::Ram16k,
};

pub struct Memory {
    ram: Ram16k,
}

impl Memory {
    pub fn power_on() -> Self {
        Self {
            ram: Ram16k::power_on(),
        }
    }

    pub fn memory(
        &mut self,           //
        input: [bool; 16],   //
        load: bool,          //
        address: [bool; 15], //
        clock: bool,         //
    ) -> [bool; 16] {
        let cb = [address[13], address[14]];
        let (
            load_ram1,     //
            load_ram2,     //
            load_screen,   //
            load_keyboard, //
        ) = demux4way(load, cb);
        let load_ram = or(load_ram1, load_ram2);

        let ram_bytes = [
            address[0],
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
            address[13],
        ];
        let ram_out = self.ram.ram16k(input, load_ram, ram_bytes, clock);

        // TODO: implement screen and keyboard
        let screen_out = [false; 16];
        let keyboard_out = [false; 16];

        // TODO: verify if this is correct
        let out = mux4way16(ram_out, ram_out, screen_out, keyboard_out, cb);

        out
    }
}
