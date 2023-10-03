mod input;
mod options;

pub struct Config {
    pub input: input::InputConfig,
    pub options: options::OptionsConfig,
}

impl Config {
    pub fn load() -> Self {
        Config {
            input: input::InputConfig::load(),
            options: options::OptionsConfig::load(),
        }
    }
}
