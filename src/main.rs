use eframe::egui;

mod gui;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "rusty picture namer gui",
        options,
        Box::new(|cc| Box::new(gui::PictureNamerGUI::new(cc))),
    );
}