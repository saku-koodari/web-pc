use crate::{
    emulated_parts::register_16bit_emulated::Register16BitEmulated,
    hack_computer::gates::gates_mw::{dmux8way, mux8way16},
};

pub struct Ram8 {
    child_circuits: [Register16BitEmulated; 8],
}

impl Ram8 {
    pub fn power_on() -> Self {
        Self {
            child_circuits: [
                // todo: remove emulation
                Register16BitEmulated::power_on(), // 1
                Register16BitEmulated::power_on(), // 2
                Register16BitEmulated::power_on(), // 3
                Register16BitEmulated::power_on(), // 4
                Register16BitEmulated::power_on(), // 5
                Register16BitEmulated::power_on(), // 6
                Register16BitEmulated::power_on(), // 7
                Register16BitEmulated::power_on(), // 8
            ],
        }
    }

    /// RAM 8
    /// Register count: 8
    pub fn ram8(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 3],
        clock: bool,
    ) -> [bool; 16] {
        let dmux_out = dmux8way(load, address);

        let out_reg0 = self.child_circuits[0].register_16bit_clocked(input, dmux_out.0, clock);
        let out_reg1 = self.child_circuits[1].register_16bit_clocked(input, dmux_out.1, clock);
        let out_reg2 = self.child_circuits[2].register_16bit_clocked(input, dmux_out.2, clock);
        let out_reg3 = self.child_circuits[3].register_16bit_clocked(input, dmux_out.3, clock);
        let out_reg4 = self.child_circuits[4].register_16bit_clocked(input, dmux_out.4, clock);
        let out_reg5 = self.child_circuits[5].register_16bit_clocked(input, dmux_out.5, clock);
        let out_reg6 = self.child_circuits[6].register_16bit_clocked(input, dmux_out.6, clock);
        let out_reg7 = self.child_circuits[7].register_16bit_clocked(input, dmux_out.7, clock);
        mux8way16(
            out_reg0, out_reg1, out_reg2, out_reg3, out_reg4, out_reg5, out_reg6, out_reg7, address,
        )
    }
}
