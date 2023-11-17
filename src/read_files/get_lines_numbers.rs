use std::io::{BufRead, BufReader};


pub fn get_lines_numbers(path: String) -> u64 {
    let my_file = std::fs::File::open(&path).expect("Couldn't read file: {path}");
    
    let reader = BufReader::new(my_file);

    
    reader.lines().count() as u64
}