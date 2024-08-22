use crate::Clipment;

mod settings;
mod tool_panel;

pub fn show_main_ui(ctx: &egui::Context, ui_data: &mut Clipment) {
    /* Content that should be shown on the main window */

    // The top tool panel which contains all the useful buttons
    tool_panel::show_tool_panel(ctx, ui_data);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Main content goes here! (The videos if you forgor)");
        for video_folder in &ui_data.video_folders {
            for video in video_folder.clone().into_iter() {
                ui.label(video.name);
            }
        }
    });

    /* Content that should be shown on it's own window */

    // Settings UI
    if ui_data.state.settings_ui {
        settings::show_settings_ui(ctx, ui_data);
    }
}