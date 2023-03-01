use crate::{
    hack_computer::{
        chips::latch::Latch,
        gates::gates_b1::{and, not, or},
    },
    utils::convert::b2n,
};

pub struct Register1Bit {
    child_circuit: [Latch; 2],
    feedback_out: bool,
}

impl Register1Bit {
    pub fn power_on() -> Self {
        Self {
            feedback_out: false,
            child_circuit: [Latch::power_on(), Latch::power_on()],
        }
    }

    pub fn reg_mux(previous_out: bool, store: bool, data: bool) -> bool {
        let and1_out = and(previous_out, not(store));
        let and2_out = and(data, store);

        or(and1_out, and2_out)
    }

    // registed is used only, when clock is active
    pub fn register_1bit_clocked(&mut self, data: bool, clock: bool, store: bool) -> bool {
        println!(
            "register_1bit_clocked(data: {}, clock: {}, store: {})",
            b2n(data),
            b2n(clock),
            b2n(store)
        );

        // selects old data or current data, whethe store bit is on.
        let selected_data = Self::reg_mux(self.feedback_out, store, data);
        println!(" - selected_data: {}", b2n(selected_data));

        // (uninuitive), use not(clock) as store indicator for the first latch
        // you could think this latch as current event,
        // that holds the bit either from input or from previous state
        let (queue_out, out_low_1) = self.child_circuit[0].d_latch(selected_data, not(clock));
        println!(
            " - queue_out: {} (out_low_1 {})",
            b2n(queue_out),
            b2n(out_low_1)
        );

        // gets the bit either from "queue" or from the clock pulse.
        let (out, out_low_2) = self.child_circuit[1].d_latch(queue_out, clock);
        println!(" - out: {} (out_low_2: {})", b2n(out), b2n(out_low_2));

        out
    }
}

mod test {
    use super::*;

    #[test]
    fn test_register_1bit_clocked() {
        let mut register = Register1Bit::power_on();
        println!("POWER ON");

        struct TestCase {
            data: bool,
            clock: bool,
            store: bool,
            expect: bool,
            test_name: &'static str,
        }
        // in order to understand, clock is expected to behave pulse-like
        // it's because the latches are triggered on the rising edge.
        // so the data wont't update until the clock-pulse triggers.
        let test_cases = vec![
            TestCase {
                data: false,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 1 - circuit inactive",
            },
            TestCase {
                data: false,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 2 - store bit activated, clock would set to false",
            },
            TestCase {
                data: false,
                clock: true,
                store: false,
                expect: false,
                // changes internal state, but does not change the output
                test_name: "test 3 - clock on, but other bits inactivated",
            },
            TestCase {
                data: false,
                clock: true,
                store: true,
                expect: false,
                test_name: "test 4 - write same data than on previous clock cycle",
            },
            TestCase {
                data: true,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 5 - set data on for the next clock cycle",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 6 - activate store, clock still off, no change",
            },
            TestCase {
                data: true,
                clock: true,
                store: true,
                expect: false,
                test_name: "test 7 - clock triggered",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 7.1 - clock untriggered",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                // note that the state change depnds of low_q:s of other circuits
                test_name: "test 8 - TODO: Verify that is this correct",
            },
            TestCase {
                data: false,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 9 - get old value",
            },
        ];

        for test in test_cases {
            println!("\n\n\n");
            println!(
                "Running test: {} - with parms data: {}, clock: {}, store: {}",
                test.test_name,
                b2n(test.data),
                b2n(test.clock),
                b2n(test.store)
            );
            let res = register.register_1bit_clocked(test.data, test.clock, test.store);

            println!(
                "\nTEST: Expected {}, and got: {}.\n",
                b2n(test.expect),
                b2n(res)
            );
            assert_eq!(
                res,
                test.expect,
                "{}",
                format!("Test failed: {}", test.test_name)
            );
        }

        println!("\n\n\n");
    }
}