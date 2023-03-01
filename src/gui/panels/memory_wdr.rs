use crate::{
    hack_computer::registers::register_16bit::Register16Bit,
    utils::{
        self,
        convert::from_string_unsigned_integer,
        convert_16b::{from_b16, from_string_integer},
    },
};

pub struct MemoryData {
    error: String,

    register: Register16Bit,
    input: String,
    store: bool,
    clock_pulse: bool,
    output: String,
}

impl Default for MemoryData {
    fn default() -> Self {
        Self {
            input: "9".to_owned(), // just some random number
            output: "<null>".to_owned(),
            store: false,
            clock_pulse: false,
            error: "".to_owned(),
            register: Register16Bit::power_on(),
        }
    }
}

pub fn panel_memory(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    data: &mut MemoryData,
    _frame: &mut eframe::Frame,
) {
    ui.label("16-Bit register");

    ui.horizontal(|ui| ui.checkbox(&mut data.store, "store bit"));
    ui.horizontal(|ui| ui.checkbox(&mut data.clock_pulse, "clock puse"));

    ui.horizontal(|ui| {
        ui.label("16-bit int:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input));
    });

    let result = from_string_integer(data.input.clone());
    match result {
        Ok(value) => {
            let output =
                data.register
                    .register_16bit(value.as_array_b16, data.store, data.clock_pulse);

            let cr = from_b16(output);
            let res = cr.unwrap();
            data.output = res.as_integer.to_string();
        }
        Err(error) => {
            data.error = format!("error: {}", error);
        }
    }

    ui.horizontal(|ui| {
        // output_out: String,
        ui.label("output");
        ui.label(data.output.clone());
    });

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
