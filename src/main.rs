use clap::Parser;

use rand::prelude::IndexedRandom;
use rand::rng;

use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

fn read_dictionary_file(filename: &PathBuf) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .to_lowercase()
        .lines()
        .map(String::from)
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>()
}

/// Random word generator
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of word combinations to produce
    #[arg(short, long, default_value_t = 1)]
    lines: usize,
    /// Number of words per line
    #[arg(short, long, default_value_t = 3)]
    number: usize,
    /// Character(s) separating words
    #[arg(short, long, default_value = ".")]
    separator: String,
    /// Path to dictionary file
    #[arg(short, long, default_value = "/usr/share/dict/words")]
    dictionary: PathBuf,
}

fn main() {
    let args = Args::parse();

    let dictionary = read_dictionary_file(&args.dictionary);

    for _ in 0..args.lines {
        let random_words = dictionary
            .sample(&mut rng(), args.number)
            .map(String::as_str)
            .collect::<Vec<&str>>();

        println!("{}", random_words.join(&args.separator));
    }
}
