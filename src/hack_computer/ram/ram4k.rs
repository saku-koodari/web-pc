use crate::hack_computer::gates::gates_mw::{dmux8way, mux8way16};

use super::ram512::Ram512;

pub struct Ram4k {
    child_parts: [Ram512; 8],
}

impl Ram4k {
    pub fn power_on() -> Self {
        Self {
            child_parts: [
                Ram512::power_on(), // 1
                Ram512::power_on(), // 2
                Ram512::power_on(), // 3
                Ram512::power_on(), // 4
                Ram512::power_on(), // 5
                Ram512::power_on(), // 6
                Ram512::power_on(), // 7
                Ram512::power_on(), // 8
            ],
        }
    }

    /// RAM 4K
    /// Rwister count: 4096
    pub fn ram4k(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 12],
        clock: bool,
    ) -> [bool; 16] {
        let dmux_out = dmux8way(load, [address[9], address[10], address[11]]);

        let a = self.child_parts[0].ram512(
            input,
            dmux_out.0,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let b = self.child_parts[1].ram512(
            input,
            dmux_out.1,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let c = self.child_parts[2].ram512(
            input,
            dmux_out.2,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let d = self.child_parts[3].ram512(
            input,
            dmux_out.3,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let e = self.child_parts[4].ram512(
            input,
            dmux_out.4,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let f = self.child_parts[5].ram512(
            input,
            dmux_out.5,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let g = self.child_parts[6].ram512(
            input,
            dmux_out.6,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );
        let h = self.child_parts[7].ram512(
            input,
            dmux_out.7,
            [
                address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                address[7], address[8],
            ],
            clock,
        );

        mux8way16(
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            [address[9], address[10], address[11]],
        )
    }
}
