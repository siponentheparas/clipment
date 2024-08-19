use std::path::PathBuf;

pub struct VideoFolder {
    pub videos: Vec<VideoInfo>,
}

pub struct VideoInfo {
    pub path: PathBuf,
    pub name: String,
    pub thumbnail: PathBuf,
}