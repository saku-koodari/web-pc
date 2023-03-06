use crate::hack_computer::gates::gates_mw::{dmux8way, mux8way16};

use super::ram8::Ram8;

pub struct Ram64 {
    child_parts: [Ram8; 8],
}

impl Ram64 {
    pub fn power_on() -> Self {
        Self {
            child_parts: [
                Ram8::power_on(), // 1
                Ram8::power_on(), // 2
                Ram8::power_on(), // 3
                Ram8::power_on(), // 4
                Ram8::power_on(), // 5
                Ram8::power_on(), // 6
                Ram8::power_on(), // 7
                Ram8::power_on(), // 8
            ],
        }
    }

    /// RAM 64
    /// Rwister count: 64
    pub fn ram64(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 6],
        clock: bool,
    ) -> [bool; 16] {
        let dmux_out = dmux8way(load, [address[3], address[4], address[5]]);

        let a = self.child_parts[0].ram8(
            input,
            dmux_out.0,
            [address[0], address[1], address[2]],
            clock,
        );
        let b = self.child_parts[1].ram8(
            input,
            dmux_out.1,
            [address[0], address[1], address[2]],
            clock,
        );
        let c = self.child_parts[2].ram8(
            input,
            dmux_out.2,
            [address[0], address[1], address[2]],
            clock,
        );
        let d = self.child_parts[3].ram8(
            input,
            dmux_out.3,
            [address[0], address[1], address[2]],
            clock,
        );
        let e = self.child_parts[4].ram8(
            input,
            dmux_out.4,
            [address[0], address[1], address[2]],
            clock,
        );
        let f = self.child_parts[5].ram8(
            input,
            dmux_out.5,
            [address[0], address[1], address[2]],
            clock,
        );
        let g = self.child_parts[6].ram8(
            input,
            dmux_out.6,
            [address[0], address[1], address[2]],
            clock,
        );
        let h = self.child_parts[7].ram8(
            input,
            dmux_out.7,
            [address[0], address[1], address[2]],
            clock,
        );

        mux8way16(a, b, c, d, e, f, g, h, [address[3], address[4], address[5]])
    }
}
