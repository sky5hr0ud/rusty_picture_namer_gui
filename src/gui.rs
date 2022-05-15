use crate::gui::picture_namer::alt_get_filetypes;
use crate::gui::picture_namer::directory_walker;
use eframe::egui;

mod picture_namer;

const VERSION: &str = "version 0.0.1";
const NAME: &str = "rusty picture namer gui ";
//const WEBSITE: &str = 

#[derive(Default)]
pub struct PictureNamerGUI {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    use_default: bool,
    list_of_filetypes_path: Option<String>,
}

impl eframe::App for PictureNamerGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let header = String::from(NAME) + VERSION;
            ui.heading(header);
            ui.label("Select a folder");

            if ui.button("Open folder...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked folder:");
                    ui.monospace(picked_path);
                });
            }

            ui.label("List of filetypes");
            ui.horizontal(|ui| {
                ui.add(egui::Checkbox::new(&mut self.use_default, "Use Alternate List of Filetypes"));
                if ui.button("Open file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.list_of_filetypes_path = Some(path.display().to_string());
                    }
                }
            });
            
            if let Some(picked_path) = &self.list_of_filetypes_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
            ui.label(format!("{}", !self.use_default));



            let filetypes = include_str!("_list_of_filetypes.txt").to_string();
            let alt_filetypes = alt_get_filetypes(filetypes);
            //directory_walker(Some(&self.picked_path), alt_filetypes.unwrap());
        });
    }
}
