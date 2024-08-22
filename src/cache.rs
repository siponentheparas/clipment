use std::fs;

use crate::file::VideoFolder;
use crate::utils::logger::*;

#[allow(dead_code)] // TODO: Remove when used
pub fn save_video_folder() -> bool {
    todo!()
}

#[allow(dead_code)] // TODO: Remove when used
pub fn load_video_folder() -> Option<Vec<VideoFolder>> {
    if let Some(cache_dir) = dirs::cache_dir() {
        let cache_folder = cache_dir.join("clipment");

        match cache_folder.try_exists() {
            Ok(exists) => {
                if exists {
                    let video_folder_json = match fs::read_to_string(cache_folder.join("video_folders.json")) {
                        Ok(json) => json,
                        Err(e) => {
                            error(&format!("Failed to read video folders cache file: {}", e));
                            return None;
                        }
                    };
    
                    let video_folder = match serde_json::from_str(&video_folder_json) {
                        Ok(video_folders) => video_folders,
                        Err(e) => {
                            error(&format!("Failed to deserialize video folders cache file: {}", e));
                            return None
                        }
                    };
                    video_folder
                } else {
                    warning("Cache folder does not exist");
                    return None
                }
            },
            Err(e) => {
                error(&format!("Failed to determine if cache folder exists: {}", e));
                return None
            }
        }
    } else {
        error("Failed to get user cache directory");
        return None
    }
}