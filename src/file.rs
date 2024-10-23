use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::logger::*;

pub mod thumbnail;

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoFolder {
    /// Name of the collection/folder
    pub name: String,

    /// Videos inside the collection/folder
    pub videos: Vec<VideoInfo>,
}

impl IntoIterator for VideoFolder {
    type Item = VideoInfo;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.videos.into_iter()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    /// Path to the video file
    pub path: PathBuf,

    /// Name of the video file
    pub name: String,

    /// Path to the thumbnail image
    pub thumbnail: PathBuf,
}

pub fn read_video_folders(video_root: PathBuf, _clip_root: PathBuf) -> Vec<VideoFolder> {
    let mut video_folders: Vec<VideoFolder> = Vec::new();

    // Video root directory
    let mut video_root_info: Vec<VideoInfo> = Vec::new();

    let video_dir = match fs::read_dir(video_root) {
        Ok(dir) => Some(dir),
        Err(e) => {
            error(&format!("Failed to read video root directory: {}", e));
            None
        }
    };

    if let Some(video_dir) = video_dir {
        for file in video_dir {
            if let Ok(file) = file {
                // If entry is not a file then continue
                if let Ok(file_type) = file.file_type() {
                    if !file_type.is_file() {
                        continue;
                    }
                }

                let file_path = file.path();

                // If file extension is not mkv or mp4 then continue
                if let Some(extension) = file_path.extension() {
                    if extension != "mkv" && extension != "mp4" {
                        continue;
                    }
                }

                if let Some(file_name) = file_path.clone().file_name() {
                    let video_info = VideoInfo {
                        path: file_path,
                        name: file_name.to_str().unwrap_or("untitled").to_string(),
                        thumbnail: PathBuf::new(),
                    };

                    info(&format!(
                        "Found video file {}",
                        &video_info.path.as_os_str().to_string_lossy()
                    ));

                    video_root_info.push(video_info);
                }
            }
        }

        let root_video_folder = VideoFolder {
            name: String::from("root"),
            videos: video_root_info,
        };

        video_folders.push(root_video_folder);
    }

    video_folders
}
