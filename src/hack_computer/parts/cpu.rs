use crate::{hack_computer::{
    chips::alu::alu,
    gates::{
        gates_b1::{and, nor, not, or},
        gates_b16::{mux16, or16},
    },
    registers::{program_counter::ProgramCounter, register_16bit::Register16Bit},
}, emulated_parts::register_16bit_emulated::Register16BitEmulated};

pub struct Cpu {
    data_out_bus: [bool; 16],
    a_register: Register16BitEmulated,
    d_register: Register16BitEmulated,
    program_counter: ProgramCounter,
}

impl Cpu {
    pub fn power_on() -> Self {
        Self {
            data_out_bus: [false; 16],
            a_register: Register16BitEmulated::power_on(),
            d_register: Register16BitEmulated::power_on(),
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
        [bool; 16], // data out bus // M - value
        bool,       // write enable // Should write into M
        [bool; 15], // (RAM) data address bus // (RAM) M - address
        [bool; 15], // (ROM) instruction address bus // (ROM) Next instruction
    ) {
        // Control bits. These are labels for each control bits
        // # address instruction
        // if instr[15] == 0 => then instruction is A instruction
        // the leftover bytes are represented as value stored in register A
        let is_a_instruction = not(instr_bus[15]);

        // # computation instructions
        // if instr[15] == 1 => then instruction is C instruction
        let is_c_instruction = instr_bus[15];

        // ## These are labeled for C instruction
        let _control_bit_x1 = instr_bus[14]; // not used
        let _control_bit_x0 = instr_bus[13]; // not used
        let control_bit_a = instr_bus[12]; // source for y input of ALU
        let control_bit_c5 = instr_bus[11]; // 1. ALU operands and computation
        let control_bit_c4 = instr_bus[10]; // 2. ALU operands and computation
        let control_bit_c3 = instr_bus[9]; // 3. ALU operands and computation
        let control_bit_c2 = instr_bus[8]; // 4. ALU operands and computation
        let control_bit_c1 = instr_bus[7]; // 5. ALU operands and computation
        let control_bit_c0 = instr_bus[6]; // 6. ALU operands and computation
        let control_bit_d2 = instr_bus[5]; // 1. destination for ALU output
        let control_bit_d1 = instr_bus[4]; // 2. destination for ALU output
        let control_bit_d0 = instr_bus[3]; // 3. destination for ALU output
        let control_bit_j2 = instr_bus[2]; // 1. jump/branch
        let control_bit_j1 = instr_bus[1]; // 2. jump/branch
        let control_bit_j0 = instr_bus[0]; // 3. jump/branch

        let (data_address_bus_16, data_address_bus_15) = self.run_a_register(
            data_bus,
            instr_bus,
            is_a_instruction,
            is_c_instruction,
            control_bit_d2,
            clock_pulse,
        );

        let data_out_bus = self.run_d_register(is_c_instruction, control_bit_d1, clock_pulse);

        let (zr, ng) = self.run_alu(
            data_out_bus,
            data_address_bus_16,
            data_bus,
            control_bit_a,
            control_bit_c5,
            control_bit_c4,
            control_bit_c3,
            control_bit_c2,
            control_bit_c1,
            control_bit_c0,
        );

        // Set bits for PC
        let next_instr = self.run_pc(
            data_address_bus_16,
            is_c_instruction,
            control_bit_j2,
            control_bit_j1,
            control_bit_j0,
            zr,
            ng,
            reset,
            clock_pulse,
        );

        // Write enable
        let write_enable = and(is_c_instruction, control_bit_d0);

        // OUT
        (
            self.data_out_bus,
            write_enable,
            data_address_bus_15,
            next_instr,
        )
    }

    fn run_a_register(
        &mut self,
        data_bus: [bool; 16],  // data bus
        instr_bus: [bool; 16], // instruction bus

        is_a_instruction: bool, // control bit
        is_c_instruction: bool, // control bit
        control_bit_d2: bool,   // control bit
        clock_pulse: bool,
    ) -> ([bool; 16], [bool; 15]) {
        // Select previous data or current instruction for register A
        let sel_a = and(is_c_instruction, control_bit_d2);
        let data_or_instr_bus = mux16(data_bus, instr_bus, sel_a);

        // Register A
        let load_a = or(is_a_instruction, sel_a);
        let data_address_bus =
            self.a_register
                .register_16bit_clocked(data_or_instr_bus, load_a, clock_pulse);

        (
            data_address_bus,
            [
                // pass two data address busses, because 16-bits are for PC and ALU
                // and 15-bits are for return the instruction address bus
                data_address_bus[0],
                data_address_bus[1],
                data_address_bus[2],
                data_address_bus[3],
                data_address_bus[4],
                data_address_bus[5],
                data_address_bus[6],
                data_address_bus[7],
                data_address_bus[8],
                data_address_bus[9],
                data_address_bus[10],
                data_address_bus[11],
                data_address_bus[12],
                data_address_bus[13],
                data_address_bus[14],
            ],
        )
    }

    fn run_d_register(
        &mut self,
        is_c_instruction: bool, // control bit
        control_bit_d1: bool,   // control bit
        clock_pulse: bool,
    ) -> [bool; 16] {
        let load_d = and(control_bit_d1, is_c_instruction);
        self.d_register
            .register_16bit_clocked(self.data_out_bus, load_d, clock_pulse)
    }

    fn run_alu(
        &mut self,
        data_out_bus: [bool; 16], // data out bus, after modification by D-register
        data_address_bus: [bool; 16], // data address bus, from A-register
        data_bus: [bool; 16],     // data bus, from input

        control_bit_a: bool,  // control bit
        control_bit_c5: bool, // control bit
        control_bit_c4: bool, // control bit
        control_bit_c3: bool, // control bit
        control_bit_c2: bool, // control bit
        control_bit_c1: bool, // control bit
        control_bit_c0: bool, // control bit
    ) -> (bool, bool) {
        // ALU input y
        let alu_in_y = mux16(data_address_bus, data_bus, control_bit_a);

        // ALU
        let (data_out_bus, zr, ng) = alu(
            data_out_bus,   // 16-bit input x
            alu_in_y,       // 16-bit input y
            control_bit_c5, // zero the x input?
            control_bit_c4, // negate the x input?
            control_bit_c3, // zero the y input?
            control_bit_c2, // negate the y input?
            control_bit_c1, // function selector
            control_bit_c0, // negate the output?
        );

        // update the data out bus
        self.data_out_bus = data_out_bus;

        (zr, ng)
    }

    fn run_pc(
        &mut self,
        data_address_bus: [bool; 16], // data bus

        is_c_instruction: bool, // control bit
        control_bit_j2: bool,   // control bit
        control_bit_j1: bool,   // control bit
        control_bit_j0: bool,   // control bit

        zr: bool, // ALU out zero flag
        ng: bool, // ALU out negative flag

        reset: bool,       // reset
        clock_pulse: bool, // clock pulse
    ) -> [bool; 15] {
        let zn = or(zr, ng);
        let is_pos = not(zn);

        let jlt = and(ng, control_bit_j2);
        let jeq = and(zr, control_bit_j1);
        let jgt = and(is_pos, control_bit_j0);

        let jle = or(jlt, jeq);
        let jmp_a = or(jle, jgt);
        let pc_load = and(is_c_instruction, jmp_a);
        let inc = not(pc_load);

        let next_instr = self.program_counter.program_counter_clocked(
            data_address_bus,
            pc_load,
            inc,
            reset,
            clock_pulse,
        );

        [
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
        ]
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
