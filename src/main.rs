use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate clap;
use clap::App;

extern crate rand;
use rand::{Rng, thread_rng};

fn main() {
    let args = App::new("wordrand")
                        .version("0.1")
                        .about("Random word generator")
                        .args_from_usage(
                            "-l, --lines=[LINES] 'Number of word combinations to produce'
                             -n, --number=[NUMBER] 'Number of words per line'
                             -s, --separator=[SEPARATOR] 'Character(s) separating words'"
                        )
                        .get_matches();

    let lines: usize = args.value_of("lines").unwrap_or("1").parse().unwrap();
    let word_count: usize = args.value_of("number").unwrap_or("3").parse().unwrap();
    let separator: &str = args.value_of("separator").unwrap_or(".");

    let dict = Path::new("/usr/share/dict/words");

    let words: Vec<String> = match File::open(&dict) {
        Ok(f) => BufReader::new(f).lines().map(|x| x.unwrap()).collect(),
        Err(message) => panic!("Couldn't open dictionary file: {}: {}", dict.display(), message)
    };

    let mut rng = thread_rng();
    let mut indices = HashSet::with_capacity(word_count);

    for _ in 0..lines {
        while indices.len() < word_count {
            indices.insert(rng.gen_range(0, words.len()));
        }
        let random_words: Vec<&str> = indices.iter().map(|i| words[*i].as_str()).collect();
        println!("{}", random_words.join(separator));
        indices.clear();
    }
} 
