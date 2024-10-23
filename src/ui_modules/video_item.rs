use crate::file::VideoInfo;
use egui::Color32;

pub fn new_video_item(video: VideoInfo, ui: &mut egui::Ui) {
    egui::Frame::none().fill(Color32::DARK_GRAY).show(ui, |ui| {
        ui.set_min_size(egui::Vec2 { x: 300.0, y: 330.0 });
        ui.set_max_size(egui::Vec2 { x: 300.0, y: 330.0 });

        ui.label(&video.name);

        if !video.thumbnail.as_os_str().is_empty() {
            ui.label("Thumbnail generated");
        }
    });
}
