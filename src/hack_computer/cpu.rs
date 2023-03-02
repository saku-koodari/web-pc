use super::{chips::alu::alu, gates::gates_b16::mux16, registers::register_16bit::Register16Bit};

pub struct Cpu {
    data_out_bus: [bool; 16],
    a_register: Register16Bit,
    d_register: Register16Bit,
}

pub struct CputOutput {
    data_out_bus: [bool; 16],    // data out bus
    write_enable: bool,          // write enable
    address_bus: [bool; 16],     // address bus
    instruction_bus: [bool; 16], // instruction bus

    // control bits (ALU)
    zr: bool, // zr
    ng: bool, // ng
}

impl Cpu {
    pub fn power_on() -> Self {
        Self {
            data_out_bus: [false; 16],
            a_register: Register16Bit::power_on(),
            d_register: Register16Bit::power_on(),
        }
    }

    // my version might require use of clock: bool;
    pub fn cpu(
        &mut self,
        instr_bus: [bool; 16],
        data_bus: [bool; 16],
        reset: bool,

        // control bits
        instr_bit: bool,
        a_reg_bit: bool,
        d_reg_bit: bool,
        alumux_bit: bool,

        // ALU control bits, note:
        // these MIGHT be same than upper ones
        zx: bool,
        nx: bool,
        zy: bool,
        ny: bool,
        f: bool,
        no: bool,

        // program counter
        pc_load: bool,
        pc_inc: bool,

        // tick tock
        clock_pulse: bool,
    ) -> CputOutput {
        let instr_out = mux16(instr_bus, self.data_out_bus, instr_bit);

        let a_reg_out = self
            .a_register
            .register_16bit_clocked(instr_out, a_reg_bit, clock_pulse);

        let alumux_out = mux16(a_reg_out, data_bus, alumux_bit);

        let d_reg_out =
            self.d_register
                .register_16bit_clocked(self.data_out_bus, d_reg_bit, clock_pulse);

        let (data_out_bus, zr, ng) = alu(d_reg_out, alumux_out, zx, nx, zy, ny, f, no);

        self.data_out_bus = data_out_bus;

        // TODO: PC
        // https://en.wikipedia.org/wiki/Hack_computer

        CputOutput {
            data_out_bus,
            zr,
            ng,
        }
    }
}
