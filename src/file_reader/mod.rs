use std::fs;

pub fn read_file(path: &String) -> String {
    let content = fs::read_to_string(path).expect("Invalid file path");
    content
}
