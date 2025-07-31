use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get() -> Option<String> {
    let file = File::open("/etc/os-release");
    if file.is_err() {
        return None;
    }
    let mut reader = BufReader::new(file.unwrap());
    let (mut line, mut pretty_name) = (String::new(), String::new());

    while reader.read_line(&mut line).expect("Failed to read line") > 0 {
        if line.starts_with("PRETTY_NAME=") {
            pretty_name = line.split_once('=').unwrap().1.to_string();
            pretty_name = pretty_name.trim().trim_matches('"').to_string();
            break;
        }
        line.clear();
    }
    Some(pretty_name)
}

