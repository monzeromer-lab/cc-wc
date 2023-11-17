use std::{fs, io};
use std::io::BufRead;

pub fn get_chars_count(path: String) -> usize {
    let my_file = fs::File::open(path).expect("Couldn't open file {path} for reading from disk");

    let reader = io::BufReader::new(my_file);

    reader.lines()
      .map(|line| line.unwrap().chars().count())
      .sum()

}