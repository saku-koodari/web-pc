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

    let input_zx = &mut data.input_zx;
    let input_nx = &mut data.input_nx;
    let input_zy = &mut data.input_zy;
    let input_ny = &mut data.input_ny;
    let input_f = &mut data.input_f;
    let input_no = &mut data.input_no;

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

    ui.horizontal(|ui| {
        ui.label(" 1-bit zx:");
        ui.horizontal(|ui| {
            ui.radio_value(input_zx, None, "0");
            if ui.radio(input_zx.is_some(), "1").clicked() {
                *input_zx = Some(FontId::default());
            }
            if let Some(input_zx) = input_zx {
                crate::introspection::font_id_ui(ui, input_zx);
            }
        });
        ui.end_row();
    });

    ui.horizontal(|ui| {
        ui.label(" 1-bit nx:");
        ui.add(egui::widgets::TextEdit::singleline(input_nx));
    });

    ui.horizontal(|ui| {
        ui.label(" 1-bit zy:");
        ui.add(egui::widgets::TextEdit::singleline(input_zy));
    });

    ui.horizontal(|ui| {
        ui.label(" 1-bit ny:");
        ui.add(egui::widgets::TextEdit::singleline(input_ny));
    });

    ui.horizontal(|ui| {
        ui.label(" 1-bit f:");
        ui.add(egui::widgets::TextEdit::singleline(input_f));
    });

    ui.horizontal(|ui| {
        ui.label(" 1-bit no:");
        ui.add(egui::widgets::TextEdit::singleline(input_no));
    });

    if ui.button("Run").clicked() {
        let result = from_string_integer(input_a.to_string())
            .and_then(|a| from_string_integer(input_b.to_string()).map(|b| (a, b)));

        match result {
            Ok((a, b)) => {
                let output_b16 = crate::pc::chips::adder::adder_b16(a.as_array_b16, b.as_array_b16);

                let output_i16 = utils::convert::from_b16(output_b16);
                data.output = output_i16.unwrap().to_string(); // TODO: Do we need to check the error?
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
