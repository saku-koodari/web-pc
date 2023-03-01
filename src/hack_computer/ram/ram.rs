use crate::hack_computer::{
    gates::gates_mw::{dmux8way, mux4way16, mux8way16},
    registers::{self, register_16bit::Register16Bit, register_1bit::Register1Bit},
};

// RAM-circuits do not have any feedback loops, so they don't require pointers,
// the registers do, and therefore they are using pointers as well
// and furthermore requires memory allocation.

pub struct Ram8 {
    child_circuits: [Register16Bit; 8],
}

impl Ram8 {
    // pub fn power_on() -> Self {
    //     Self {
    //         child_circuits: [],
    //         //child_circuits: [Register16Bit::power_on(); 8],
    //     }
    // }
    /*
        /// RAM 8
        /// Register count: 8
        pub fn ram8(&mut self, input: [bool; 16], load: bool, address: [bool; 3]) -> [bool; 16] {
            let dmux_out = dmux8way(load, address);

            let out_reg0 = self.child_circuits[0].register_16bit(input, dmux_out.0);
            let out_reg1 = self.child_circuits[1].register_16bit(input, dmux_out.1);
            let out_reg2 = self.child_circuits[2].register_16bit(input, dmux_out.2);
            let out_reg3 = self.child_circuits[3].register_16bit(input, dmux_out.3);
            let out_reg4 = self.child_circuits[4].register_16bit(input, dmux_out.4);
            let out_reg5 = self.child_circuits[5].register_16bit(input, dmux_out.5);
            let out_reg6 = self.child_circuits[6].register_16bit(input, dmux_out.6);
            let out_reg7 = self.child_circuits[7].register_16bit(input, dmux_out.7);
            mux8way16(
                out_reg0, out_reg1, out_reg2, out_reg3, out_reg4, out_reg5, out_reg6, out_reg7, address,
            )
        }
    }

    pub struct Ram64 {
        child_parts: [Ram8; 8],
    }

    impl Ram64 {
        pub fn power_on() -> Self {
            Self {
                child_parts: [Ram8::power_on(); 8],
            }
        }

        /// RAM 64
        /// Rwister count: 64
        pub fn ram64(&mut self, input: [bool; 16], load: bool, address: [bool; 6]) -> [bool; 16] {
            let dmux_out = dmux8way(load, [address[3], address[4], address[5]]);

            let a = self.child_parts[0].ram8(input, dmux_out.0, [address[0], address[1], address[2]]);
            let b = self.child_parts[1].ram8(input, dmux_out.1, [address[0], address[1], address[2]]);
            let c = self.child_parts[2].ram8(input, dmux_out.2, [address[0], address[1], address[2]]);
            let d = self.child_parts[3].ram8(input, dmux_out.3, [address[0], address[1], address[2]]);
            let e = self.child_parts[4].ram8(input, dmux_out.4, [address[0], address[1], address[2]]);
            let f = self.child_parts[5].ram8(input, dmux_out.5, [address[0], address[1], address[2]]);
            let g = self.child_parts[6].ram8(input, dmux_out.6, [address[0], address[1], address[2]]);
            let h = self.child_parts[7].ram8(input, dmux_out.7, [address[0], address[1], address[2]]);

            mux8way16(a, b, c, d, e, f, g, h, [address[3], address[4], address[5]])
        }
    }

    pub struct Ram512 {
        child_parts: [Ram64; 8],
    }

    impl Ram512 {
        pub fn power_on() -> Self {
            Self {
                child_parts: [Ram64::power_on(); 8],
            }
        }

        /// RAM 512
        /// Rwister count: 512
        pub fn ram512(&mut self, input: [bool; 16], load: bool, address: [bool; 9]) -> [bool; 16] {
            let dmux_out = dmux8way(load, [address[6], address[7], address[8]]);

            let a = self.child_parts[0].ram64(
                input,
                dmux_out.0,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let b = self.child_parts[1].ram64(
                input,
                dmux_out.1,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let c = self.child_parts[2].ram64(
                input,
                dmux_out.2,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let d = self.child_parts[3].ram64(
                input,
                dmux_out.3,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let e = self.child_parts[4].ram64(
                input,
                dmux_out.4,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let f = self.child_parts[5].ram64(
                input,
                dmux_out.5,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let g = self.child_parts[6].ram64(
                input,
                dmux_out.6,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );
            let h = self.child_parts[7].ram64(
                input,
                dmux_out.7,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
            );

            mux8way16(a, b, c, d, e, f, g, h, [address[6], address[7], address[8]])
        }
    }

    pub struct Ram4k {
        child_parts: [Ram512; 8],
    }

    impl Ram4k {
        pub fn power_on() -> Self {
            Self {
                child_parts: [Ram512::power_on(); 8],
            }
        }

        /// RAM 4K
        /// Rwister count: 4096
        pub fn ram4k(&mut self, input: [bool; 16], load: bool, address: [bool; 12]) -> [bool; 16] {
            let dmux_out = dmux8way(load, [address[9], address[10], address[11]]);

            let a = self.child_parts[0].ram512(
                input,
                dmux_out.0,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let b = self.child_parts[1].ram512(
                input,
                dmux_out.1,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let c = self.child_parts[2].ram512(
                input,
                dmux_out.2,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let d = self.child_parts[3].ram512(
                input,
                dmux_out.3,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let e = self.child_parts[4].ram512(
                input,
                dmux_out.4,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let f = self.child_parts[5].ram512(
                input,
                dmux_out.5,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let g = self.child_parts[6].ram512(
                input,
                dmux_out.6,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
            );
            let h = self.child_parts[7].ram512(
                input,
                dmux_out.7,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5], address[6],
                    address[7], address[8],
                ],
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

    pub struct Ram16k {
        child_parts: [Ram4k; 4],
    }

    impl Ram16k {
        pub fn power_on() -> Self {
            Self {
                child_parts: [Ram4k::power_on(); 4],
            }
        }

        /// RAM 16K
        /// Rwister count: 16384
        pub fn ram16k(&mut self, input: [bool; 16], load: bool, address: [bool; 14]) -> [bool; 16] {
            let dmux_out = dmux8way(load, [address[11], address[12], address[13]]);

            print!("\n");
            println!("RAM 16k");
            println!("MUX OUT: {:?}", dmux_out);

            let a = self.child_parts[0].ram4k(
                input,
                dmux_out.0,
                [
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
                ],
            );
            let b = self.child_parts[1].ram4k(
                input,
                dmux_out.1,
                [
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
                ],
            );
            let c = self.child_parts[2].ram4k(
                input,
                dmux_out.2,
                [
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
                ],
            );
            let d = self.child_parts[3].ram4k(
                input,
                dmux_out.3,
                [
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
                ],
            );

            println!("RAM 4k - a: {:?}", a);
            println!("RAM 4k - b: {:?}", b);
            println!("RAM 4k - c: {:?}", c);
            println!("RAM 4k - d: {:?}", d);
            print!("\n");

            mux4way16(a, b, c, d, [address[12], address[13]])
        }
    */
}

mod test {
    use crate::utils::{
        convert::from_string_unsigned_integer,
        convert_16b::{from_b16, from_i16},
    };
    /*
    #[test]
    fn test_ram16k() {
        fn addr(str: &str) -> [bool; 14] {
            from_string_unsigned_integer::<14>(String::from(str.to_owned()))
                .unwrap()
                .as_array_b_nsize
        }

        // app created, so ram should be empty
        let mut ram16k = super::Ram16k::power_on();
        let expect = from_i16(0).unwrap();
        let input = from_i16(500).unwrap().as_array_b16;
        let load = false;
        let address = addr("0");

        // Act
        let output = ram16k.ram16k(input, load, address);
        let conv = from_b16(output).unwrap();

        // Assert
        print!("\n");
        println!("LEFT = ACTUAL");
        println!("RIGHT = EXPECTED");
        assert_eq!(conv.to_string(), expect.to_string());
    }
    */
}