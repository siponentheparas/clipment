#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

mod user_settings;
mod utils;

fn main() -> Result<(), eframe::Error> {
    utils::logger::info("App started");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 625.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Clipment",
        options,
        Box::new(|_cc| Box::<Clipment>::default()),
    )
}

struct Clipment {
    settings: user_settings::UserSettings,
}

impl Default for Clipment {
    fn default() -> Self {
        Clipment {
            settings: user_settings::UserSettings::load().unwrap_or_default(),
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
        });
    }
}