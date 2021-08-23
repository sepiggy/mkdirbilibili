use std::env;
use std::fs::DirBuilder;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let splits: Vec<&str> = arg.split("/").collect();
    let video_name_index = splits[4].find("?").unwrap();
    let video_name = splits[4].get(0..video_name_index).unwrap();
    let dir_name = "bilibili-".to_string() + video_name;
    let path = Path::new(".").join(dir_name);
    DirBuilder::new().recursive(true).create(path).unwrap();
}
