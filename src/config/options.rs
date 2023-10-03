use std::env;

pub struct OptionsConfig {
    pub trim: bool,
}

impl OptionsConfig {
    pub fn load() -> Self {
        OptionsConfig { trim: true }
    }
}
