use crate::hack_computer::{
    chips::alu::alu,
    gates::{
        gates_b1::{and, nor, not, or},
        gates_b16::mux16,
    },
    registers::{program_counter::ProgramCounter, register_16bit::Register16Bit},
};

pub struct Cpu {
    data_out_bus: [bool; 16],
    a_register: Register16Bit,
    d_register: Register16Bit,
    program_counter: ProgramCounter,
    increment_pc: bool, // this could be used for debugging
}

impl Cpu {
    pub fn power_on() -> Self {
        Self {
            increment_pc: true,
            data_out_bus: [false; 16],
            a_register: Register16Bit::power_on(),
            d_register: Register16Bit::power_on(),
            program_counter: ProgramCounter::power_on(),
        }
    }

    // my version might require use of clock: bool;
    pub fn cpu(
        &mut self,
        instr_bus: [bool; 16],
        data_bus: [bool; 16],
        reset: bool,

        // tick tock
        clock_pulse: bool, // TODO: Some HDL's I saw, does not have this. Could I avoid using this?
    ) -> (
        [bool; 16], // data out bus
        bool,       // write enable
        [bool; 15], // data address bus
        [bool; 15], // instruction address bus
    ) {
        // Control bits. These are labels for each control bits
        let cb_j1 = instr_bus[0];
        let cb_j2 = instr_bus[1];
        let cb_j3 = instr_bus[2];
        let cb_d3 = instr_bus[3];
        let cb_d2 = instr_bus[4];
        let cb_d1 = instr_bus[5];
        let cb_c6 = instr_bus[6];
        let cb_c5 = instr_bus[7];
        let cb_c4 = instr_bus[8];
        let cb_c3 = instr_bus[9];
        let cb_c2 = instr_bus[10];
        let cb_c1 = instr_bus[11];
        let cb_load_reg_a = instr_bus[12];
        // instr_bus[13] not used
        // instr_bus[14] not used
        let use_instruction = instr_bus[15];

        // first Part
        let reg_a_in = mux16(instr_bus, data_bus, use_instruction);

        let not_instruction = not(instr_bus[15]);

        // Register A
        let load_a_reg = or(cb_d1, not_instruction);
        let reg_a_out = self
            .a_register
            .register_16bit_clocked(reg_a_in, load_a_reg, clock_pulse);

        let alu_in_y = mux16(reg_a_out, data_bus, cb_d1);

        // Register D
        let load_d_reg = and(cb_d2, use_instruction);
        let alu_in_x =
            self.d_register
                .register_16bit_clocked(self.data_out_bus, load_d_reg, clock_pulse);

        // ALU
        let zx = and(cb_c1, use_instruction);
        let nx = and(cb_c2, use_instruction);
        let zy = or(cb_c3, not_instruction);
        let ny = or(cb_c4, not_instruction);
        let f = and(cb_c5, use_instruction);
        let no = and(cb_c6, use_instruction);

        let (data_out_bus, zr, ng) = alu(alu_in_x, alu_in_y, zx, nx, zy, ny, f, no);
        self.data_out_bus = data_out_bus;

        // Write enable
        let enable_write = and(cb_d3, use_instruction);

        // PC
        let pc_pos = nor(zr, ng);

        let pc_j3 = and(cb_j3, pc_pos);
        let pc_j2 = and(cb_j2, zr);
        let pc_j1 = and(cb_j1, ng);

        // TODO: This could be optimized. Or could it?
        let pc_j12 = or(pc_j2, pc_j1);
        let pc_j123 = or(pc_j12, pc_j3);

        let jump = and(pc_j123, use_instruction);

        let next_instr = self.program_counter.program_counter_clocked(
            reg_a_out,
            jump,
            self.increment_pc,
            reset,
            clock_pulse,
        );

        // OUT
        (
            data_out_bus,
            enable_write,
            [
                // data address bus
                reg_a_out[0],
                reg_a_out[1],
                reg_a_out[2],
                reg_a_out[3],
                reg_a_out[4],
                reg_a_out[5],
                reg_a_out[6],
                reg_a_out[7],
                reg_a_out[8],
                reg_a_out[9],
                reg_a_out[10],
                reg_a_out[11],
                reg_a_out[12],
                reg_a_out[13],
                reg_a_out[14],
            ],
            [
                // instruction address bus
                next_instr[0],
                next_instr[1],
                next_instr[2],
                next_instr[3],
                next_instr[4],
                next_instr[5],
                next_instr[6],
                next_instr[7],
                next_instr[8],
                next_instr[9],
                next_instr[10],
                next_instr[11],
                next_instr[12],
                next_instr[13],
                next_instr[14],
            ],
        )
    }

    pub fn get_debug_info(&mut self) -> ([bool; 16], [bool; 16], [bool; 16]) {
        (
            self.a_register.get_debug_info(),
            self.d_register.get_debug_info(),
            self.program_counter.get_debug_info(),
        )
    }
}

mod test {
    #[test]
    fn test_cpu() {
        use super::*;
        let mut cpu = Cpu::power_on();

        let instr_bus = [false; 16];
        let data_bus = [false; 16];
        let reset = false;
        let clock_pulse = false;

        // clock systems are difficult to test due to synchronizing
        // therefore it's better to test in the UI
        // and here just verify that the components don't fall in pieces.

        let out = cpu.cpu(instr_bus, data_bus, reset, clock_pulse);

        assert_eq!(out.0, [false; 16]);
        assert_eq!(out.1, false);
        assert_eq!(out.2, [false; 15]);
    }
}
