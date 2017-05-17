use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
extern crate rand;

use rand::{thread_rng, sample};
fn main() {
    let word_count = 3;

    let dict = Path::new("/usr/share/dict/words");
    let file = match File::open(&dict) {
        Ok(f) => f,
        Err(message) => panic!("Couldn't open dictionary file: {}: {}", dict.display(), message)
    };

    let words: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut rng = thread_rng();
    let random_words = sample(&mut rng, words, word_count);
}
