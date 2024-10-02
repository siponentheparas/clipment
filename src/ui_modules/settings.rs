use std::path::PathBuf;

use crate::{utils::logger, Clipment};

pub fn show_settings_ui(ctx: &egui::Context, ui_data: &mut Clipment) {
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_height(200.0);
            ui.set_width(300.0);

            let videos_path = ui_data.temp_settings.videos_root_path.clone().unwrap_or(PathBuf::new());

            ui.label("Source video clip files location:").highlight();
            ui.add_space(3.0);
            ui.label(format!("Path: {}", videos_path.to_string_lossy()));
            if ui.button("Change").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Change Source Video Location")
                    .set_directory(videos_path)
                    .pick_folder()
                {
                    ui_data.temp_settings.videos_root_path = Some(path);
                }
            }

            ui.add_space(10.0);

            let clips_path = ui_data.temp_settings.clips_path.clone().unwrap_or(PathBuf::new());

            ui.label("Clips folder to save clips to");
            ui.label(format!("Path: {}", clips_path.to_string_lossy()));
            if ui.button("Change").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                .set_title("Change clips folder")
                .set_directory(clips_path)
                .pick_folder()
                {
                    ui_data.temp_settings.clips_path = Some(path);
                }
            }

            egui::TopBottomPanel::bottom("save_cancel").exact_height(20.0).show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("Cancel").clicked() {
                        logger::info("Settings canceled, not saving.");
                        ui_data.state.settings_ui = false;
                    }

                    ui.add_space(200.0);

                    if ui.button("Save").clicked() {
                        ui_data.settings = ui_data.temp_settings.clone();
                        logger::info("Cloned temp settings to settings");

                        ui_data.settings.save();
        
                        ui_data.state.settings_ui = false;
                        logger::info("Closed settings ui");
                    }
                });
            });
        });
}
