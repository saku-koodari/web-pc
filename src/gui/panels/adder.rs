use crate::utils::{self, convert::from_string};

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
    frame: &mut eframe::Frame,
) {
    ui.label("16-bit Adder");
    ui.horizontal(|ui| {
        ui.label("Input A:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_a));
    });

    ui.horizontal(|ui| {
        ui.label("Input B:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_b));
    });

    if ui.button("Calculate").clicked() {
        let result = from_string(&data.input_a)
            .and_then(|a| from_string(&data.input_b).map(|b| (a.bin_array, b.bin_array)));

        match result {
            Ok((a, b)) => {
                let output_b16 = crate::pc::chips::adder::adder_rca_lsb_b16(a, b);

                let output_i32 = utils::convert::from_b16(output_b16.0);
                data.output = output_i32.to_string();

                if output_b16.1 {
                    data.error = "Overflow!".to_owned();
                } else {
                    data.error = "".to_owned();
                }
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
