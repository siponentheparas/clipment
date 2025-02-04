use crate::file::VideoInfo;
use egui::Color32;

pub fn new_video_item(video: VideoInfo, ui: &mut egui::Ui) {
    egui::Frame::none().fill(Color32::DARK_GRAY).show(ui, |ui| {
        ui.set_min_size(egui::Vec2 { x: 300.0, y: 330.0 });
        ui.set_max_size(egui::Vec2 { x: 300.0, y: 330.0 });

        ui.add_sized([ui.available_width(), 5.0], egui::Label::new(&video.name));

        if !video.thumbnail.as_os_str().is_empty() {
            ui.add(
                egui::Image::new(format!(
                    "file://{}",
                    video.thumbnail.as_os_str().to_string_lossy()
                ))
                .max_width(280.0)
                .max_height(300.0)
                .rounding(5.0),
            )
            .on_hover_text(video.thumbnail.as_os_str().to_string_lossy())
            .paint_debug_info();
        } else {
            ui.label("Loading thumbnail...");
        }
    });
}
