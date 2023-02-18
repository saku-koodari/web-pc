pub fn side_panel(ui: &mut egui::Ui, label: &mut String, value: &mut f32, frame: &mut eframe::Frame) {
    ui.heading("My Side Panel");

    ui.horizontal(|ui| {
        ui.label("Write something: ");
        ui.text_edit_singleline(label);
    });

    ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
    if ui.button("Increment").clicked() {
        *value += 1.0;
    }

    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
    });

    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
    egui::Button::new("Quit").on_click(move |_| {
        frame.close();
    }).ui(ui);
}
