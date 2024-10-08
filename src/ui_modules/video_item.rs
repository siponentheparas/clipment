use std::fs::DirBuilder;

use crate::file::{self, VideoInfo};
use crate::utils::logger::*;
use dirs::cache_dir;
use egui::Color32;

pub fn new_video_item(video: VideoInfo, ui: &mut egui::Ui) {
    egui::Frame::none().fill(Color32::DARK_GRAY).show(ui, |ui| {
        ui.set_min_size(egui::Vec2 { x: 300.0, y: 330.0 });
        ui.set_max_size(egui::Vec2 { x: 300.0, y: 330.0 });

        ui.label(&video.name);

        if ui.button("gen").clicked() {
            let thumbnail = file::thumbnail::generate_thumbnail(&video);

            if let Some(mut cache_dir) = cache_dir() {
                cache_dir = cache_dir.join("clipment/thumb");

                match cache_dir.try_exists() {
                    Ok(exists) => {
                        if !exists {
                            if let Err(e) = DirBuilder::new().recursive(true).create(&cache_dir) {
                                error(&format!("Failed to create thumbnail cache folder: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        error(&format!(
                            "Failed to determine if thumbnail cache folder exists: {}",
                            e
                        ));
                        return;
                    }
                }

                if let Err(e) = file::thumbnail::save_thumbnail_to_file(
                    thumbnail,
                    cache_dir.join(format!("{}.png", video.name)),
                ) {
                    error(&format!("Failed to save thumbnail to file: {}", e));
                }
            } else {
                error("Failed to get user cache dir");
            }
        }
    });
}
