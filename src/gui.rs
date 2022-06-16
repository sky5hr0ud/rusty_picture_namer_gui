use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};

mod picture_namer;

const VERSION: &str = "version 1.0.2";
const NAME: &str = "rusty picture namer gui ";
const YEAR: &str = "2022";
const GITHUB: &str = "https://github.com/sky5hr0ud/rusty_picture_namer_gui";
const DEFAULT_FILETYPES: &str = ".jpg .jpeg .png .mp4 .dng .gif .nef .bmp .jpe .jif .jfif .jfi .webp .tiff .tif .psd .raw .arw .cr2 .nrw .k25 .dib .heif .heic .ind .indd .indt .jp2 .j2k .jpf .jpx .jpm .mj2 .svg .svgz .ai .eps .pdf .xcf .cdr .sr2 .orf .bin .afphoto .mkv .mov";

#[derive(Default)]
pub struct PictureNamerGUI {
    picked_path: Option<String>,
    folder_selected: bool,
    use_alternate: bool,
    list_of_filetypes_path: Option<String>,
    result: (bool, String),
}

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn big_button() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(50.0, Proportional)),
        (heading2(), FontId::new(30.0, Proportional)),
        (TextStyle::Body, FontId::new(25.0, Proportional)),
        (TextStyle::Monospace, FontId::new(18.0, Proportional)),
        (TextStyle::Button, FontId::new(18.0, Proportional)),
        (big_button(), FontId::new(30.0, Proportional)),
        (TextStyle::Small, FontId::new(18.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl PictureNamerGUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Self {
            picked_path: Some(String::from("")),
            folder_selected: false,
            use_alternate: false,
            list_of_filetypes_path: Some(String::from("")),
            result: (false, String::from("Filenamer has not ran.")),
        }
    }
}

/// This is the GUI code.
impl eframe::App for PictureNamerGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|header| {
                header.heading(NAME);
                header.small(VERSION);
                header.small(YEAR);
                header.hyperlink_to("Github", GITHUB);
            });

            ui.separator();

            ui.horizontal(|select_folder| {
                select_folder.label(RichText::new("Select a folder").text_style(heading2()).strong()).on_hover_ui(|hover_ui| {
                    hover_ui.monospace("Select a folder containing files to be renamed");
                });
                if select_folder.button("Open Folder...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                    self.folder_selected = true;
                    }
                };
            });

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|selected_folder| {
                    selected_folder.set_visible(self.folder_selected);
                    selected_folder.label("Selected folder:");
                    selected_folder.monospace(picked_path);
                });
            }

            ui.separator();

            ui.horizontal(|filetypes_text| {
                filetypes_text.label(RichText::new("Filetypes").text_style(heading2()).strong());
                filetypes_text.set_visible(!self.use_alternate);
                filetypes_text.label("(Default)").on_hover_ui(|hover_ui| {
                    hover_ui.monospace(DEFAULT_FILETYPES);
                });
                filetypes_text.collapsing("Click to see default filetypes...", |default_filetypes| {
                    default_filetypes.horizontal_wrapped(|filetypes| {
                        filetypes.monospace(DEFAULT_FILETYPES);
                    });
                });
            });

            ui.horizontal(|alt_filetypes| {
                alt_filetypes.add(egui::Checkbox::new(&mut self.use_alternate, "Use Alternate List of Filetypes"));
                alt_filetypes.add_enabled_ui(self.use_alternate, |open_file| {
                    if open_file.button("Open File...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.list_of_filetypes_path = Some(path.display().to_string());
                        }
                    }
                });
            });

            ui.horizontal(|alt_filetypes| {
                alt_filetypes.set_visible(self.use_alternate);
                if let Some(picked_path) = &self.list_of_filetypes_path {
                    alt_filetypes.horizontal(|selected_filetypes| {
                        selected_filetypes.label("Selected file:");
                        selected_filetypes.monospace(picked_path);
                    });
                };
            });

            ui.separator();

            ui.add_enabled_ui(self.folder_selected, |start| {
                if start.button(RichText::new("Start!").text_style(big_button()).strong()).clicked() {
                    self.result = pass_to_picture_namer(&self.picked_path, self.use_alternate, &self.list_of_filetypes_path);
                }
                start.monospace(self.result.1.clone());
            });
        });
    }
}

fn pass_to_picture_namer(picked_path: &Option<String>, use_alternate: bool, list_of_filetypes_path: &Option<String>) -> (bool, String){
    let mut path_vec = Vec::new();
    if let Some(picked_path) = picked_path {
        path_vec.push(picked_path.to_string());
    }
    if let Some(list_of_filetypes_path) = list_of_filetypes_path {
        path_vec.push(list_of_filetypes_path.to_string());
    }
    picture_namer::picture_namer_set_state(path_vec, use_alternate)
}
