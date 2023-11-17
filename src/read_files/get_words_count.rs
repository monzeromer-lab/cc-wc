use std::io::BufRead;

pub fn get_words_count(path: String) -> usize {
    let  my_file = std::fs::File::open(&path).expect("Couldn't read file: {path}");

    let reader = std::io::BufReader::new(my_file);

    reader.lines()
      .map(|line| line.unwrap().split_whitespace().count())
      .sum()
}