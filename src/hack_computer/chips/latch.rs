use crate::hack_computer::gates::gates_b1::{nand, not};

// This struct is called Circuit because it represents the circuit.
// When you run SR NAND latch or D Flip Flop, the electric current is trapped in the circuits.
// To digitally trap the voltage (in the circuit), it's stored in the struct.
pub struct Latch {
    // represents the ouputs of SR NAND latch
    pub prev_q_high: bool,
    pub prev_q_low: bool,
}

impl Latch {
    pub fn power_on() -> Latch {
        Latch {
            prev_q_high: false,
            prev_q_low: true,
        }
    }

    // https://en.wikipedia.org/wiki/Flip-flop_(electronics)
    /// SR NAND latch
    ///
    /// The circuit uses feedback to "remember" and retain its logical state even after the controlling input signals have changed. When the S and R inputs are both high, feedback maintains the Q outputs to the previous state.
    ///
    /// However, feedback loop is not possible, so we need to do this differently.
    /// The difference is that the previous value of Q is stored at circuit struct.
    ///
    ///  The truth table:
    ///
    /// | set | reset | Q high  |  Q low  |     notes      |
    /// |-----|-------|---------|---------|----------------|
    /// |  0  |   0   |    1    |    1    | not allowed    |
    /// |  0  |   1   |    1    |    0    | Q = 1          |
    /// |  1  |   0   |    0    |    1    | Q = 0          |
    /// |  1  |   1   | Q(t-1)+ | Q(t-1)- | Q = previous Q |
    // -set------------+------+
    //                 │ NAND ┝ -Q high---┬---
    //          ┌------+------+           │
    //          │  ┌----------------------┘
    //          │  ┃
    //          └--╂----------------------┐
    //             └--+------+            |
    //                │ NAND ┝ -Q low-----┴---
    // -reset---------+------+
    pub fn sr_nand_latch(&mut self, set: bool, reset: bool) -> (bool, bool) {
        // values are initialzied at Circuit::new()
        // run the circuit
        if !set && !reset {
            panic!("Either set or reset must be on in order to make it this circuit run properly.");
        }

        let q_high = nand(set, self.prev_q_low);
        let q_low = nand(self.prev_q_high, reset);

        // Q high and low are changed, update.
        self.prev_q_high = q_high;
        self.prev_q_low = q_low;

        (self.prev_q_high, self.prev_q_low)
    }

    /// Digital latch
    // inputs:
    //
    // data  O---┬----------+------+   +-------+
    //           ┃          │ NAND ┝---|       |- Q high
    //           ┃   ┌------+------+   |  SR   |
    //           ┃   │                 | NAND  |
    // store O---╂---┴------+------+   | LATCH |
    //           ┃          │ NAND ┝---|       |- Q low
    //           ┗━ +NOT----+------+   +-------+
    //
    pub fn d_latch(&mut self, data: bool, store: bool) -> (bool, bool) {
        // store can also mean clock
        let set = nand(data, store);
        let reset = nand(not(data), store);

        // Q high and Q low are not needed here, but Q high to return
        // This also mean that that this function is now stated and requires
        // Circuit struct
        self.sr_nand_latch(set, reset)
    }
}

mod test {
    #[test]
    #[should_panic]
    fn test_sr_nand_latch_wrong_params_panic() {
        use super::Latch;
        let mut circuit = Latch::power_on();

        circuit.sr_nand_latch(false, false);
    }

    #[test]
    fn test_sr_nand_latch() {
        struct TestCase {
            set: bool,
            reset: bool,
            expect_q_high: bool,
            expect_q_low: bool,
        }

        use super::Latch;
        let mut circuit = Latch::power_on();

        let test_cases = vec![
            TestCase {
                // note: is not possible, that both Q_HIGH and Q_LOW mutates at the same time.
                // atleast that's how this is expected to behave.
                // Reason: buld the gate and try to switch both at the same time.
                set: true,
                reset: false,
                expect_q_high: false,
                expect_q_low: true,
            },
            TestCase {
                set: false,
                reset: true,
                expect_q_high: true, // this should never happen???
                expect_q_low: true,
            },
            TestCase {
                set: true,
                reset: true,
                expect_q_high: false,
                expect_q_low: false,
            },
        ];

        let mut i = 0;
        for test in test_cases {
            println!("Test case: {}", i);
            i += 1;

            let (q_high, q_low) = circuit.sr_nand_latch(test.set, test.reset);
            assert_eq!(q_high, test.expect_q_high);
            assert_eq!(q_low, test.expect_q_low);
        }
    }

    #[test]
    fn integration_test_digital_flipflop() {
        struct TestCase {
            data: bool,
            store: bool,
            expect_out: bool,
        }

        use super::Latch;

        // In order to use flip flop, you need to initialize it in the code.
        // This basically happens, turn the power supply of the circuit on.
        let mut circuit = Latch::power_on();

        // Note: this is an integration test
        let test_cases = vec![
            TestCase {
                data: false,
                store: false,
                expect_out: false,
            },
            TestCase {
                data: false,
                store: true,
                expect_out: false,
            },
            TestCase {
                data: true,
                store: false,
                expect_out: false,
            },
            TestCase {
                data: true,
                store: true,
                expect_out: true,
            },
        ];

        println!("\n\n\n\n\n\n\nLET'S RUN THE TESTS!\n\n");
        for case in test_cases {
            for i in 0..1 {
                let expected_out = case.expect_out;
                let data = case.data;
                let store = case.store;

                // Let's also simulate,
                // that user most likely can't press buttons at the speed of light.
                // The circuit receives the same event several times.
                let (actual_out, _) = circuit.d_latch(data, store);

                println!("circuit Q high: {}", circuit.prev_q_high);
                println!("circuit Q low: {}", circuit.prev_q_low);

                println!("round {i}: digital_flipflop(data:{data}, store:{store}) -> actual:{actual_out}, expected:{expected_out}\n");
                assert_eq!(actual_out, case.expect_out);
            }
        }
    }
}
