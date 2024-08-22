use std::{fs::{self, DirBuilder}, path::PathBuf};
use serde::{Serialize, Deserialize};
use dirs::{config_dir, video_dir};

use crate::utils::logger::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub videos_root_path: Option<PathBuf>,
    pub clips_path: Option<PathBuf>,
}

impl Default for UserSettings {
    fn default() -> Self {
        info("Creating default user settings");

        let videos_path: Option<PathBuf>;
        let clips_path: Option<PathBuf>;

        if let Some(video_path) = video_dir() {
            videos_path = Some(video_path.clone());

            clips_path = Some(video_path.join("clips"));
        } else {
            error("Failed to get user video directory");
            warning("videos root path is set to None");
            videos_path = None;
            clips_path = None;
        }

        UserSettings {
            videos_root_path: videos_path,
            clips_path,
        }
    }
}

impl UserSettings {
    pub fn save(&self) {
        info("Saving settings");

        if let Some(mut config_path) = config_dir() {
            config_path.push("clipment");

            // Create settings folder `.config/clipment/` if it does not exist
            // If error happens, don't save and just return. Otherwise the code will become too nested and unreadable.
            match config_path.try_exists() {
                Ok(exists) => {
                    if !exists {
                        warning("Settings folder does not exist");
                        if let Err(e) = DirBuilder::new()
                        .recursive(true)
                        .create(&config_path) {
                            error(&format!("Failed to create settings folder\n
                            Folder path: {}\n
                            Error: {}", config_path.to_string_lossy(), e));
                            error("Settings not saved!");
                            return;
                        } else {
                            info("Created settings folder");
                        }
                    }
                },
                Err(e) => {
                    error(&format!("Failed to determine settings folder existence.\n
                    Folder path: {}\n
                    Error: {}", config_path.to_string_lossy(), e));
                    error("Settings not saved!");
                    return;
                }
            }

            // Save to settings.json file.
            config_path.push("settings.json");

            let settings_json = serde_json::to_string_pretty(&self).unwrap();

            if let Err(e) = fs::write(&config_path, settings_json) {
                error(&format!("Failed to write to settings file\n
                File path: {}\n
                Error: {}", config_path.to_string_lossy(), e));
                error("Settings not saved!");
            } else {
                info(&format!("Settings saved to {}", config_path.to_string_lossy()));
            }
        } else {
            error("Failed to determine user settings path.");
            error("Settings not saved!");
        }
    }

    pub fn load() -> Option<Self>{
        if let Some(config) = config_dir() {
            let config_path = config.join("clipment/settings.json");

            if let Ok(exists) = config_path.try_exists() {
                if !exists {
                    warning("User settings file does not exist");
                    return None;
                }
            }

            let settings_json = match fs::read_to_string(config_path) {
                Ok(json) => json,
                Err(e) => {
                    error(&format!("Error while reading user settings file: {}", e));
                    return None;
                }
            };

            let settings: UserSettings = match serde_json::from_str(&settings_json) {
                Ok(user_settings) => user_settings,
                Err(e) => {
                    error("Error while reading user settings json. File might be corrupt or tampered with.");
                    error(&e.to_string());
                    return None;
                }
            };

            return Some(settings);
        } else {
            error("Failed to determine user settings path.");
            return None;
        }
    }
}