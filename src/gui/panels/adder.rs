use crate::utils;

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

pub fn panelAdder(
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut AdderData,
    frame: &mut eframe::Frame,
) {
    ui.horizontal(|ui| {
        ui.label("Input A:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_a));
    });

    ui.horizontal(|ui| {
        ui.label("Input B:");
        ui.add(egui::widgets::TextEdit::singleline(&mut data.input_b));
    });

    if ui.button("Calculate").clicked() {
        let parse_a_res = data.input_a.parse::<i32>();
        let parse_b_res = data.input_b.parse::<i32>();

        if parse_a_res.is_err() {
            data.error = "Input A is not a valid integer!".to_owned();
            return;
        }

        if parse_b_res.is_err() {
            data.error = "Input B is not a valid integer!".to_owned();
            return;
        }

        let a_i32 = parse_a_res.unwrap();
        let a_b16 = utils::convert::int_to_b16_lsb(a_i32);

        let b_i32 = parse_b_res.unwrap();
        let b_b16 = utils::convert::int_to_b16_lsb(b_i32);

        let output_b16 = crate::pc::chips::adder::adder_rca_lsb_b16(a_b16, b_b16);

        let output_i32 = utils::convert::b16_to_int_lsb(output_b16.0);
        data.output = output_i32.to_string();

        if output_b16.1 {
            data.error = "Overflow!".to_owned();
        } else {
            data.error = "".to_owned();
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
