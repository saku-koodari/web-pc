use crate::{
    hack_computer::registers::register_1bit::Register1Bit,
    utils::{self, convert::from_string_unsigned_integer, convert_16b::from_string_integer},
};

pub struct MemoryData {
    error: String,

    register: Register1Bit,
    load: bool,
    store: bool,
    clock_pulse: bool,
    output: bool,
}

impl Default for MemoryData {
    fn default() -> Self {
        Self {
            load: false,
            store: false,
            clock_pulse: false,
            output: false,
            error: "".to_owned(),
            register: Register1Bit::power_on(),
        }
    }
}

pub fn panel_memory(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    data: &mut MemoryData,
    _frame: &mut eframe::Frame,
) {
    ui.label("1-Bit register");

    ui.horizontal(|ui| ui.checkbox(&mut data.load, "load bit"));
    ui.horizontal(|ui| ui.checkbox(&mut data.store, "store bit"));
    ui.horizontal(|ui| ui.checkbox(&mut data.clock_pulse, "clock puse"));

    data.output = data
        .register
        .register_1bit_clocked(data.load, data.clock_pulse, data.store);
    ui.horizontal(|ui| ui.checkbox(&mut data.output, "load bit"));

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
