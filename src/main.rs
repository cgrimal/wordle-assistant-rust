use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_letters(words: Vec<String>) -> HashMap<char, u32> {
    let character_list: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").collect();

}

fn score_words(words: Vec<String>) -> HashMap<String, u32> {
    return words.into_iter().map(|word| (word, 3)).collect()
}

fn main() {
    let path = "./data/dictionary_fr_sutom.txt";
    let word_length = 7;

    let mut word_list: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // lines.iter().for_each(|line| println!("{}", line));

    println!("Words count: {}", word_list.len());

    word_list = word_list.into_iter().filter(|word| word.len() == word_length).collect();

    println!("Words count: {}", word_list.len());

    let words_score: HashMap<String, u32> = score_words(word_list);

    let best_word = words_score.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("Best word: {} ({})", best_word.0, best_word.1);

    // words_score.iter().for_each(|(word, score)| println!("{} -> {}", word, score));

}
