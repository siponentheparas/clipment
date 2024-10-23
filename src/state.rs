// Struct to keep track of which ui modules to show and stuff
pub struct State {
    // TODO: Make state save/loadable
    // UI modules
    pub settings_ui: bool,

    // Status messages. To show the user what the program is doing.
    // e.g. generating thumbnails for videos.
    pub generating_thumb: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            settings_ui: false,

            generating_thumb: true,
        }
    }
}
