use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate clap;
use clap::{App};

extern crate rand;
use rand::{thread_rng, sample};

fn main() {
    let args = App::new("wordrand")
                        .version("0.1")
                        .about("Random word generator")
                        .args_from_usage(
                            "-n, --number=[NUMBER] 'Number of words per line'"
                        )
                        .get_matches();

    let word_count: usize = args.value_of("lines").unwrap_or("3").parse().unwrap();

    let dict = Path::new("/usr/share/dict/words");

    let file = match File::open(&dict) {
        Ok(f) => f,
        Err(message) => panic!("Couldn't open dictionary file: {}: {}", dict.display(), message)
    };

    let words: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut rng = thread_rng();
    let random_words = sample(&mut rng, words, word_count);

    
    println!("{}", random_words.join("."));

}
