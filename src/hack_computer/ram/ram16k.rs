use crate::{
    emulated_parts::register_16bit_emulated::Register16BitEmulated,
    hack_computer::{
        gates::gates_mw::{dmux8way, mux4way16, mux8way16},
        registers::{self, register_16bit::Register16Bit, register_1bit::Register1Bit},
    },
};

use super::ram4k::Ram4k;

// RAM-circuits do not have any feedback loops, so they don't require pointers,
// the registers do, and therefore they are using pointers as well
// and furthermore requires memory allocation.

/// RAM that fits 16384 words. The RAM is word addressable only.
pub struct Ram16k {
    child_parts: [Ram4k; 4],
}

impl Ram16k {
    pub fn power_on() -> Self {
        Self {
            child_parts: [
                Ram4k::power_on(), // 1
                Ram4k::power_on(), // 2
                Ram4k::power_on(), // 3
                Ram4k::power_on(), // 4
            ],
        }
    }

    /// RAM 16K
    /// Rwister count: 16384
    pub fn ram16k(
        &mut self,
        input: [bool; 16],
        load: bool,
        address: [bool; 14],
        clock: bool,
    ) -> [bool; 16] {
        let dmux_out = dmux8way(load, [address[11], address[12], address[13]]);

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
            clock,
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
            clock,
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
            clock,
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
            clock,
        );

        mux4way16(a, b, c, d, [address[12], address[13]])
    }
}

mod test {
    use crate::utils::{
        convert::from_string_unsigned_integer,
        convert_16b::{from_b16, from_i16},
    };

    #[test]
    fn test_ram16k() {
        fn addr<const N: usize>(my_str: &str) -> [bool; N] {
            from_string_unsigned_integer::<N>(String::from(my_str.to_owned()))
                .unwrap()
                .as_array_b_nsize
        }

        // app created, so ram should be empty
        let mut ram16k = super::Ram16k::power_on();
        let expect = from_i16(0).unwrap();
        let input = from_i16(500).unwrap().as_array_b16;
        let load = false;
        let clock = false;
        let address = addr("0");

        // Act
        let output = ram16k.ram16k(input, load, address, clock);
        let conv = from_b16(output).unwrap();

        // Assert
        print!("\n");
        println!("LEFT = ACTUAL");
        println!("RIGHT = EXPECTED");
        assert_eq!(conv.to_string(), expect.to_string());
    }
}
