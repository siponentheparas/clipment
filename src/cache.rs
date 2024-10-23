use std::fs::{self, DirBuilder};

use dirs::cache_dir;
use serde_json;

use crate::file::VideoFolder;
use crate::utils::logger::*;

#[allow(dead_code)] // TODO: Remove when used
pub fn save_video_folder(video_folders: &Vec<VideoFolder>) -> bool {
    let mut cache_file_path = if let Some(path) = cache_dir() {
        path
    } else {
        error("Failed to get cache directory path");
        return false;
    };

    cache_file_path.push("clipment");

    match cache_file_path.try_exists() {
        Ok(exists) => {
            if !exists {
                if let Err(e) = DirBuilder::new().recursive(true).create(&cache_file_path) {
                    error(&format!("Failed to create cache directory: {}", e));
                }
            }
        }
        Err(e) => {
            error(&format!(
                "Failed to determine if cache directory exists: {}",
                e
            ));
        }
    }

    cache_file_path.push("video_folders.json");

    let video_folders_json;

    match serde_json::to_string(video_folders) {
        Ok(json) => video_folders_json = json,
        Err(e) => {
            error(&format!(
                "Failed to serialize video folders into json: {}",
                e
            ));
            return false;
        }
    }

    if let Err(e) = fs::write(cache_file_path, video_folders_json) {
        error(&format!(
            "Failed to write video_folders.json to cache: {}",
            e
        ));
        return false;
    }

    return true;
}

#[allow(dead_code)] // TODO: Remove when used
pub fn load_video_folder() -> Option<Vec<VideoFolder>> {
    if let Some(cache_dir) = cache_dir() {
        let cache_folder = cache_dir.join("clipment");

        match cache_folder.try_exists() {
            Ok(exists) => {
                if exists {
                    let video_folder_json =
                        match fs::read_to_string(cache_folder.join("video_folders.json")) {
                            Ok(json) => json,
                            Err(e) => {
                                error(&format!("Failed to read video folders cache file: {}", e));
                                return None;
                            }
                        };

                    let video_folder = match serde_json::from_str(&video_folder_json) {
                        Ok(video_folders) => video_folders,
                        Err(e) => {
                            error(&format!(
                                "Failed to deserialize video folders cache file: {}",
                                e
                            ));
                            return None;
                        }
                    };
                    video_folder
                } else {
                    warning("Cache folder does not exist");
                    return None;
                }
            }
            Err(e) => {
                error(&format!(
                    "Failed to determine if cache folder exists: {}",
                    e
                ));
                return None;
            }
        }
    } else {
        error("Failed to get user cache directory");
        return None;
    }
}
