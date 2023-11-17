mod read_files;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, )]
struct Args {
    /// Name of the file
    #[arg(name = "FILE")]
    f: Option<String>,

    /// print the byte counts
    #[arg(short, long)]
    c: Option<String>,

    /// print the line counts
    #[arg(short, long)]
    l: Option<String>,

    /// print the word counts
    #[arg(short, long)]
    w: Option<String>,

    /// print the character counts
    #[arg(short, long)]
    m: Option<String>,
}

fn main() {
    let args = Args::parse();
 
    if let Some(some_file) = args.c.as_deref() {
        // println!("got file: {:?}", some_file);
        let size = read_files::get_file_size::read_file_size(some_file.to_string());
        println!("{:?}", size);
    }

    if let Some(some_file) = args.l.as_deref() {
        // println!("got file: {:?}", some_file);
        let size: u64 = read_files::get_lines_numbers::get_lines_numbers(some_file.to_string());
        println!("{:?}", size);
    }

    if let Some(some_file) = args.w.as_deref() {
        // println!("got file: {:?}", some_file);
        let size: usize = read_files::get_words_count::get_words_count(some_file.to_string());
        println!("{:?}", size);
    }

    if let Some(some_file) = args.m.as_deref() {
        // println!("got file: {:?}", some_file);
        let size: usize = read_files::get_chars_count::get_chars_count(some_file.to_string());
        println!("{:?}", size);
    }

    if let Some(some_file) = args.f.as_deref() {

        let file_size = read_files::get_file_size::read_file_size(some_file.to_string());
        println!("file size is: {:?}", file_size);

        let file_lines = read_files::get_lines_numbers::get_lines_numbers(some_file.to_string());
        println!("file lines is: {:?}", file_lines);

        let file_words = read_files::get_words_count::get_words_count(some_file.to_string());
        println!("file words is: {:?}", file_words);

        let file_chars = read_files::get_chars_count::get_chars_count(some_file.to_string());
        println!("file chars is: {:?}", file_chars);
    }
}
