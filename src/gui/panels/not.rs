use egui::Context;

use crate::utils::{self, convert_16b::from_string_integer};

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

pub fn panel_not(
    _ctx: &Context,
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut NotData,
    _frame: &mut eframe::Frame,
) {
    ui.label(label.clone());
    ui.horizontal(|ui| {
        ui.label("Input:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input));

        let label_result = utils::convert_16b::from_string_integer(data.input.clone());
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
        let result = from_string_integer(data.input.clone());
        match result {
            Ok(a) => {
                let output_b16 = crate::pc::gates::gates_b16::not16(a.as_array_b16);

                let output = utils::convert_16b::from_b16(output_b16);
                data.output = output.unwrap().to_string(); // TODO: Do we need to check the error?
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
