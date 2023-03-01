use crate::utils::{self, convert_16b::from_string_integer};

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
    _frame: &mut eframe::Frame,
) {
    let input_x = &mut data.input_x;
    let input_y = &mut data.input_y;

    ui.label(label.clone());
    ui.label("input:");

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

    if ui.button("Run").clicked() {
        println!("CLICK");
        let result = from_string_integer(data.input_x.clone()).and_then(|a| {
            from_string_integer(data.input_y.clone()).map(|b| (a.as_array_b16, b.as_array_b16))
        });

        match result {
            Ok((a, b)) => {
                let alu_res = crate::hack_computer::chips::alu::alu(
                    a,
                    b,
                    data.input_zx,
                    data.input_nx,
                    data.input_zy,
                    data.input_ny,
                    data.input_f,
                    data.input_no,
                );

                let output_i32 = utils::convert_16b::from_b16(alu_res.0);
                data.output_out = output_i32.unwrap().to_string(); // TODO: Do we need to check the error?
                data.output_zr = alu_res.1;
                data.output_ng = alu_res.2;
            }
            Err(e) => {
                data.error = e;
                return;
            }
        }
    }

    ui.horizontal(|ui| ui.label("Result:"));

    let label_result = utils::convert_16b::from_string_integer(data.output_out.clone());
    match label_result {
        Ok(a) => {
            ui.horizontal(|ui| {
                // output_zr: bool,
                ui.label("zr:");
                ui.label(data.output_zr.to_string());

                // output_ng: bool,
                ui.label("ng:");
                ui.label(data.output_ng.to_string());
            });

            ui.horizontal(|ui| {
                // output_out: String,
                ui.label("16-bit out (int):");
                ui.label(a.as_integer.to_string());
            });

            ui.horizontal(|ui| {
                // output_out: String,
                ui.label("16-bit out (hex):");
                ui.label(a.as_string_hex.to_string());
            });

            ui.horizontal(|ui| {
                // output_out: String,
                ui.label("16-bit out (bin):");
                ui.label(a.as_string_bin.to_string());
            });

            ui.horizontal(|ui| {
                // output_out: String,
                ui.label("16-bit out (internal b16):");
                ui.label(format!("{:?}", a.as_array_b16));
            });
        }

        Err(e) => {
            ui.horizontal(|ui| {
                ui.label(format!("Error: {}", e));
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
