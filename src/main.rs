use std::collections::HashMap;

mod keyword_parser;

fn main() {
    let keywords: HashMap<String, u32> = keyword_parser::process_keywords(
        String::from("data/keywords.txt")
    ).unwrap();

    for entry in keywords {
        let (key, score) = entry;
        println!("{}: {}", key, score);
    }
}