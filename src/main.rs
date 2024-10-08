#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;
use std::env;

use eframe::egui;
use file::VideoFolder;

mod user_settings;
mod utils;
mod ui_modules;
mod state;
mod cache;
mod file;

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
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 625.0]),
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
    }
}