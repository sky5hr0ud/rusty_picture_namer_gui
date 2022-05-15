mod gui;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "rusty_picture_namer",
        options,
        Box::new(|_cc| Box::new(gui::PictureNamerGUI::default())),
    );
}