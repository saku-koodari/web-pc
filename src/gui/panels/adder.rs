pub fn panelAdder(ui: &mut egui::Ui, label: &mut String, value: &mut f32, frame: &mut eframe::Frame) {
    //this is a textbox
    let mut input_a = String::from("1");
    let mut input_b = String::from("2");
    let mut output = String::from("3");
    ui.horizontal(|ui| {
        ui.label("a:");
        ui.text_edit_singleline(&mut input_a);
    });

    ui.horizontal(|ui| {
        ui.label("b:");
        ui.text_edit_singleline(&mut input_b);
    });

    ui.horizontal(|ui| {
        ui.label("out:");
        ui.text_edit_singleline(&mut output);
    });
}
