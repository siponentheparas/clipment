// Struct to keep track of which ui modules to show and stuff
pub struct State { // TODO: Make state save/loadable
    // UI modules
    pub settings_ui: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            settings_ui: false,
        }    
    }
}