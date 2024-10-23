#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::env;
use std::path::PathBuf;

use dirs::cache_dir;
use eframe::egui;
use file::VideoFolder;
use file::VideoInfo;
use std::fs::DirBuilder;
use utils::logger::*;

mod cache;
mod file;
mod state;
mod ui_modules;
mod user_settings;
mod utils;

#[cfg(debug_assertions)]
fn backtrace() {
    env::set_var("RUST_BACKTRACE", "1");
}

#[cfg(not(debug_assertions))]
fn backtrace() {
    env::set_var("RUST_BACKTRACE", "1");
}

fn main() -> Result<(), eframe::Error> {
    backtrace();

    utils::logger::info("App started");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 621.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Clipment",
        options,
        Box::new(|_cc| Ok(Box::<Clipment>::default())),
    )
}

struct Clipment {
    state: state::State,
    settings: user_settings::UserSettings,
    /// Temporary user settings, used for text edit fields.
    /// When changed settings are saved, this temporary settings will be copied to real settings.
    temp_settings: user_settings::UserSettings,
    video_folders: Vec<VideoFolder>,
}

impl Default for Clipment {
    fn default() -> Self {
        let settings = user_settings::UserSettings::load().unwrap_or_default();
        let video_root = settings.videos_root_path.clone().unwrap_or(PathBuf::new());
        let clips_path = settings.clips_path.clone().unwrap_or(PathBuf::new());
        Clipment {
            state: state::State::default(),
            settings: settings.clone(),
            temp_settings: settings,
            video_folders: file::read_video_folders(video_root, clips_path),
        }
    }
}

impl eframe::App for Clipment {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Do stuff when window close is requested
        if ctx.input(|i| i.viewport().close_requested()) {
            self.settings.save();
            utils::logger::info("Closing window");
        }

        // Show the main UI
        ui_modules::show_main_ui(ctx, self);

        // TODO: Move into a thread!
        if self.state.generating_thumb {
            for (ivf, video_folder) in self.video_folders.clone().iter().enumerate() {
                for (iv, video) in video_folder.clone().into_iter().enumerate() {
                    if let Some(path) = generate_thumbnail(video) {
                        self.video_folders[ivf].videos[iv].thumbnail = path;
                    }
                }
            }

            cache::save_video_folder(&self.video_folders);

            self.state.generating_thumb = false;
        }
    }
}

// TODO: Move this maybe?
fn generate_thumbnail(video: VideoInfo) -> Option<PathBuf> {
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
                return None;
            }
        }

        let video_file_path = cache_dir.join(format!("{}.png", video.name));
        if let Err(e) = file::thumbnail::save_thumbnail_to_file(thumbnail, &video_file_path) {
            error(&format!("Failed to save thumbnail to file: {}", e));
        }

        return Some(video_file_path);
    } else {
        error("Failed to get user cache dir");
        return None;
    }
}
