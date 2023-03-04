pub struct Rom {}

impl Rom {
    pub fn power_on() -> Self {
        Self {}
    }

    pub fn rom(&mut self, instruction: [bool; 16]) -> [bool; 16] {
        [false; 16]
    }
}
