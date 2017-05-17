use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let dict = Path::new("/usr/share/dict/words");
    let file = match File::open(&dict) {
        Ok(f) => f,
        Err(message) => panic!("Couldn't open dictionary file: {}: {}", dict.display(), message)
    };

    let words: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
}
