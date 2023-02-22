use crate::utils::{self, convert::from_string_integer};

pub struct AluData {
    // TODO: Is it possible to convert into i16?
    input_x: String,
    input_y: String,
    input_zx: bool,
    input_nx: bool,
    input_zy: bool,
    input_ny: bool,
    input_f: bool,
    input_no: bool,

    output_out: String,
    output_zr: bool,
    output_ng: bool,

    error: String,
}

impl Default for AluData {
    fn default() -> Self {
        Self {
            input_x: "0".to_owned(),
            input_y: "0".to_owned(),
            input_zx: false,
            input_nx: false,
            input_zy: false,
            input_ny: false,
            input_f: false,
            input_no: false,

            output_out: "0".to_owned(),
            output_zr: false,
            output_ng: false,

            error: "".to_owned(),
        }
    }
}

pub fn panel_alu(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut AluData,
    frame: &mut eframe::Frame,
) {
    let input_x = &mut data.input_x;
    let input_y = &mut data.input_y;

    ui.label("16-bit ALU");
    ui.label("inputs:");

    ui.horizontal(|ui| {
        ui.label("16-bit int - X:");
        ui.add(egui::widgets::TextEdit::singleline(input_x));
    });

    ui.horizontal(|ui| {
        ui.label("16-bit int - Y:");
        ui.add(egui::widgets::TextEdit::singleline(input_y));
    });

    ui.horizontal(|ui| ui.label("Control bits:"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_zx, "zx"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_nx, "nx"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_zy, "zy"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_ny, "ny"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_f, "f"));
    ui.horizontal(|ui| ui.checkbox(&mut data.input_no, "no"));

    if ui.button("Run").clicked() {}

    ui.horizontal(|ui| ui.label("Result:"));
    ui.horizontal(|ui| {
        ui.label("out:");

        let label_result = utils::convert::from_string_integer(data.output_out.clone());
        match label_result {
            Ok(a) => {
                ui.label(a.to_string());
            }
            Err(e) => {
                ui.label(format!("Error: {}", e));
            }
        }
    });

    // output_out: String,
    // output_zr: bool,
    // output_ng: bool,

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
