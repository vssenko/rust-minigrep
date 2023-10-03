mod config;
mod content_search;
mod file_reader;

fn main() {
    let config = config::Config::load();
    let file_content = file_reader::read_file(&config.input.file_path);
    let result = content_search::search_content(&file_content, &config.input.regexp);
    for r in result {
        println!("{}", r);
    }
}
