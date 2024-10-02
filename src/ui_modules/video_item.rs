use std::path::PathBuf;

use egui::Color32;
use crate::file::{self, VideoInfo};

pub fn new_video_item(video: VideoInfo, ui: &mut egui::Ui) {
    egui::Frame::none().fill(Color32::DARK_GRAY).show(ui, |ui| {
        ui.set_min_size(egui::Vec2 { x: 300.0, y: 330.0 });
        ui.set_max_size(egui::Vec2 { x: 300.0, y: 330.0 });

        ui.label(&video.name);

        if ui.button("gen").clicked() {
            let thumbnail = file::thumbnail::generate_thumbnail(&video);

            file::thumbnail::save_thumbnail_to_file(thumbnail, PathBuf::from(format!("/home/siponen/Documents/{}.png", video.name)));
        }
    });
}