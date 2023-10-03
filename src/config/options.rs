use std::env;

pub struct OptionsConfig {
    pub trim: bool,
    pub trim_size: Option<usize>,
}

impl OptionsConfig {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();

        let trim = args.iter().position(|arg| arg == "--trim");

        let trim_enabled = trim.is_some();
        let mut trim_size = None;

        if trim_enabled && trim.unwrap() < args.len() {
            let next_arg_after_trim = args.get(trim.unwrap() + 1);
            if let Some(arg) = next_arg_after_trim {
                let parse_result = arg.parse::<usize>();
                if (parse_result.is_ok()) {
                    trim_size = Some(parse_result.unwrap());
                }
            }
        }

        OptionsConfig {
            trim: trim_enabled,
            trim_size,
        }
    }
}
