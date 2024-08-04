use crate::{utils::logger, Clipment};

pub fn show_tool_panel(ctx: &egui::Context, ui_data: &mut Clipment) {
    egui::TopBottomPanel::top("tool_panel").exact_height(30.0).show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            if ui.button("Settings").clicked() {
                ui_data.temp_settings = ui_data.settings.clone();
                logger::info("Copied temp_settings from settings");

                ui_data.state.settings_ui = true;
                logger::info("Showing settings ui");
            }
        });        
    });
}