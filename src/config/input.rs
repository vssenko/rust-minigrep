use std::env;

pub struct InputConfig {
    pub file_path: String,
    pub regexp: String,
}

impl InputConfig {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            panic!("Not enough arguments!")
        }

        InputConfig {
            file_path: args[1].clone(),
            regexp: args[2].clone(),
        }
    }
}
