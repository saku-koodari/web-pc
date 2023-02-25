use crate::utils::{self, convert_16b::from_string_integer};

pub struct AdderData {
    // TODO: Is it possible to convert into i32?
    input_a: String,
    input_b: String,
    output: String,
    error: String,
}

impl Default for AdderData {
    fn default() -> Self {
        Self {
            input_a: "0".to_owned(),
            input_b: "0".to_owned(),
            output: "0".to_owned(),
            error: "".to_owned(),
        }
    }
}

pub fn panel_adder(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut AdderData,
    _frame: &mut eframe::Frame,
) {
    ui.label(label.clone());
    ui.horizontal(|ui| {
        ui.label("Input A:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_a));
    });

    ui.horizontal(|ui| {
        ui.label("Input B:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_b));
    });

    if ui.button("Calculate").clicked() {
        let result = from_string_integer(data.input_a.clone()).and_then(|a| {
            from_string_integer(data.input_b.clone()).map(|b| (a.as_array_b16, b.as_array_b16))
        });

        match result {
            Ok((a, b)) => {
                let output_b16 = crate::hack_computer::chips::adder::adder_b16(a, b);

                let output_i32 = utils::convert_16b::from_b16(output_b16);
                data.output = output_i32.unwrap().to_string(); // TODO: Do we need to check the error?
            }
            Err(e) => {
                data.error = e;
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
