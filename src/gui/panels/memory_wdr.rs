use crate::utils::{self, convert::from_string_unsigned_integer, convert_16b::from_string_integer};
use crate::hack_computer::ram::ram::Ram16k;

pub struct MemoryData {
    // TODO: Is it possible to convert into i16?
    address: String,
    input: String,
    load: bool,
    output: String,

    error: String,
}

impl Default for MemoryData {
    fn default() -> Self {
        Self {
            address: "0".to_owned(),
            input: "0".to_owned(),
            load: false,
            output: "0".to_owned(),
            error: "".to_owned(),
        }
    }
}

pub fn panel_memory(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    data: &mut MemoryData,
    _frame: &mut eframe::Frame,
) {
    ui.label("Memory with data race issues");

    ui.horizontal(|ui| {
        ui.label("16-bit int:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input));
    });

    ui.horizontal(|ui| {
        ui.label("14-bit address as int (0 - 16383):");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.address));
    });

    ui.horizontal(|ui| ui.label("Load bit:"));
    ui.horizontal(|ui| ui.checkbox(&mut data.load, "load"));

    if ui.button("Go to next state").clicked() {
        println!("CLICK");
        let result = from_string_integer(data.input.clone()).and_then(|a| {
            from_string_unsigned_integer::<14>(data.address.clone())
                .map(|b| (a.as_array_b16, b.as_array_b_nsize))
        });

        match result {
            Ok((input, address)) => {
                let mut ram16k = Ram16k::power_on();

                let mem_res = ram16k.ram16k(input, data.load, address);

                let output_i16 = utils::convert_16b::from_b16(mem_res);
                data.output = output_i16.unwrap().as_integer.to_string(); // TODO: Do we need to check the error?
            }
            Err(e) => {
                data.error = "ERROR: ".to_owned() + &e;
                return;
            }
        }
    }

    ui.horizontal(|ui| ui.label("Result:"));

    let output_conv_res = utils::convert_16b::from_string_integer(data.output.clone());
    match output_conv_res {
        Ok(output) => {
            ui.horizontal(|ui| {
                // output
                ui.label("output:");
                ui.label(output.to_string());
            });
        }

        Err(e) => {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Error when converting {} The error: {}",
                    data.output.clone(),
                    e
                ));
            });
        }
    }

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
