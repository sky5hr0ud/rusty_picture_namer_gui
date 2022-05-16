#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

mod gui;

/// Opens a GUI titled "rusty picture namer gui".
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