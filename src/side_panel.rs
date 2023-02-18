use egui::menu::SubMenu;

pub fn side_panel(
    ui: &mut egui::Ui,
    label: &mut String,
    value: &mut f32,
    frame: &mut eframe::Frame,
) {
    use egui::{menu, Button};
    ui.heading("Tools");
    ui.label("Chips: 16-bit Adder");

    //this is a textbox
    let mut input_a = String::from("0");
    let mut input_b = String::from("0");
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
        ui.text_edit_singleline(&mut input_b);
    });

    // this is a slider
    // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));

    // this is a button
    // if ui.button("Increment").clicked() {
    //     *value += 1.0;
    // }

    // this is a footer
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("The source on ");
            ui.hyperlink_to("github", "https://github.com/saku-kaarakainen/web-pc");
            ui.label(".");
        });
    });
}
