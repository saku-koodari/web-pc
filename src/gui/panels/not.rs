use egui::Context;

use crate::utils::{self, convert::from_string};

pub struct NotData {
    input: String,
    output: String,
    error: String,
}

impl Default for NotData {
    fn default() -> Self {
        Self {
            input: "0".to_owned(),
            output: "0".to_owned(),
            error: "".to_owned(),
        }
    }
}

pub fn panelNot(
    ctx: &Context,
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut NotData,
    frame: &mut eframe::Frame,
) {
    ui.label("16-bit Adder");
    ui.horizontal(|ui| {
        ui.label("Input:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input));

        let label_result = utils::convert::from_string(&data.input);
        match label_result {
            Ok(a) => {
                ui.label(a.to_string());
            }
            Err(e) => {
                ui.label(format!("Error: {}", e));
            }
        }
    });

    if ui.button("Calculate").clicked() {
        let result = from_string(&data.input);
        match result {
            Ok(a) => {
                let output_b16 = crate::pc::gates::gates_b16::not16(a.bin_array);

                let output = utils::convert::from_b16(output_b16);
                data.output = output.to_string();
            }
            Err(e) => {
                data.output = format!("{}", e);
                data.error = format!("{}", e);
                return;
            }
        }
    }

    ui.horizontal(|ui| {
        ui.label("Result:");
        ui.add(egui::widgets::Label::new(format!("{}", data.output)));
    });

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
