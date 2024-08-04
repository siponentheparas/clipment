use std::{fs::{self, DirBuilder}, path::PathBuf};
use serde::{Serialize, Deserialize};
use dirs::{config_dir, video_dir};

use crate::utils::logger;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub videos_root_path: Option<PathBuf>,
}

impl Default for UserSettings {
    fn default() -> Self {
        logger::info("Creating default user settings");

        let videos_path: Option<PathBuf>;

        if let Some(video_path) = video_dir() {
            videos_path = Some(video_path);
        } else {
            logger::error("Failed to get user video directory");
            logger::warning("videos root path is set to None");
            videos_path = None;
        }

        UserSettings {
            videos_root_path: videos_path,
        }
    }
}

impl UserSettings {
    pub fn save(&self) {
        logger::info("Saving settings");

        if let Some(mut config_path) = config_dir() {
            config_path.push("clipment");

            // Create settings folder `.config/clipment/` if it does not exist
            // If error happens, don't save and just return. Otherwise the code will become too nested and unreadable.
            match config_path.try_exists() {
                Ok(exists) => {
                    if !exists {
                        logger::warning("Settings folder does not exist");
                        if let Err(e) = DirBuilder::new()
                        .recursive(true)
                        .create(&config_path) {
                            logger::error(&format!("Failed to create settings folder\n
                            Folder path: {}\n
                            Error: {}", config_path.to_string_lossy(), e));
                            logger::error("Settings not saved!");
                            return;
                        } else {
                            logger::info("Created settings folder");
                        }
                    }
                },
                Err(e) => {
                    logger::error(&format!("Failed to determine settings folder existence.\n
                    Folder path: {}\n
                    Error: {}", config_path.to_string_lossy(), e));
                    logger::error("Settings not saved!");
                    return;
                }
            }

            // Save to settings.json file.
            config_path.push("settings.json");

            let settings_json = serde_json::to_string_pretty(&self).unwrap();

            if let Err(e) = fs::write(&config_path, settings_json) {
                logger::error(&format!("Failed to write to settings file\n
                File path: {}\n
                Error: {}", config_path.to_string_lossy(), e));
                logger::error("Settings not saved!");
            } else {
                logger::info(&format!("Settings saved to {}", config_path.to_string_lossy()));
            }
        } else {
            logger::error("Failed to determine user settings path.");
            logger::error("Settings not saved!");
        }
    }

    pub fn load() -> Option<Self>{
        if let Some(config) = config_dir() {
            let config_path = config.join("clipment/settings.json");

            let settings_json = match fs::read_to_string(config_path) {
                Ok(json) => json,
                Err(e) => {
                    logger::error(&format!("Error while reading user settings file: {}", e));
                    return None;
                }
            };

            let settings: UserSettings = match serde_json::from_str(&settings_json) {
                Ok(user_settings) => user_settings,
                Err(e) => {
                    logger::error("Error while reading user settings json. File might be corrupt or tampered with.");
                    logger::error(&e.to_string());
                    return None;
                }
            };

            return Some(settings);
        } else {
            logger::error("Failed to determine user settings path.");
            return None;
        }
    }
}