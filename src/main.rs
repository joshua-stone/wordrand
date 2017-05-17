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
                            "-l, --lines=[LINES] 'Number of word combinations to produce'
                             -n, --number=[NUMBER] 'Number of words per line'
                             -s, --separator=[SEPARATOR] 'Character(s) separating words'"
                        )
                        .get_matches();

    let lines = args.value_of("lines").unwrap_or(" ").parse().unwrap();
    let word_count: usize = args.value_of("number").unwrap_or("3").parse().unwrap();
    let separator: &str = args.value_of("separator").unwrap_or(".");

    let dict = Path::new("/usr/share/dict/words");

    let file = match File::open(&dict) {
        Ok(f) => f,
        Err(message) => panic!("Couldn't open dictionary file: {}: {}", dict.display(), message)
    };

    let words: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut rng = thread_rng();

    for _ in 0..lines { 
        let random_words = sample(&mut rng, words.iter().map(|s| s.as_ref()), word_count);
        println!("{}", random_words.join(separator));
    }
}
