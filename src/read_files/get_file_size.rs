use std::{self, fs};

pub fn read_file_size(path: String) -> u64 {
    let my_file = fs::metadata(&path).expect("couldn't read {path}");

    my_file.len()
}
