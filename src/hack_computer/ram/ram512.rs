use crate::hack_computer::gates::gates_mw::{dmux8way, mux8way16};

use super::ram64::Ram64;

pub struct Ram512 {
    child_parts: [Ram64; 8],
}

impl Ram512 {
    pub fn power_on() -> Self {
        Self {
            child_parts: [
                Ram64::power_on(), // 1
                Ram64::power_on(), // 2
                Ram64::power_on(), // 3
                Ram64::power_on(), // 4
                Ram64::power_on(), // 5
                Ram64::power_on(), // 6
                Ram64::power_on(), // 7
                Ram64::power_on(), // 8
            ],
        }
    }

    /// RAM 512
    /// Rwister count: 512
    pub fn ram512(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 9],
        clock: bool,
    ) -> [bool; 16] {
        let dmux_out = dmux8way(load, [address[6], address[7], address[8]]);

        let a = self.child_parts[0].ram64(
            input,
            dmux_out.0,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let b = self.child_parts[1].ram64(
            input,
            dmux_out.1,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let c = self.child_parts[2].ram64(
            input,
            dmux_out.2,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let d = self.child_parts[3].ram64(
            input,
            dmux_out.3,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let e = self.child_parts[4].ram64(
            input,
            dmux_out.4,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let f = self.child_parts[5].ram64(
            input,
            dmux_out.5,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let g = self.child_parts[6].ram64(
            input,
            dmux_out.6,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );
        let h = self.child_parts[7].ram64(
            input,
            dmux_out.7,
            [
                address[0], address[1], address[2], address[3], address[4], address[5],
            ],
            clock,
        );

        mux8way16(a, b, c, d, e, f, g, h, [address[6], address[7], address[8]])
    }
}
