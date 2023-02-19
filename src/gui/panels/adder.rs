pub struct AdderData {
    // TODO: Is it possible to convert into i32?
    input_a: String,
    input_b: String,
    output: String,
}

impl Default for AdderData {
    fn default() -> Self {
        Self {
            input_a: "0".to_owned(),
            input_b: "0".to_owned(),
            output: "0".to_owned(),
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
        let a = data.input_a.parse::<i32>().unwrap();
        let b = data.input_b.parse::<i32>().unwrap();
        data.output = (a + b).to_string();
    }

    ui.horizontal(|ui| {
        ui.label("Result:");
        ui.add(egui::widgets::Label::new(format!("{}", data.output)));
    });
}
