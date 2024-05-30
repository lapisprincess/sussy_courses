use std::collections::HashMap;

pub mod keyword_parser;

fn main() {
    let keywords: HashMap<String, u32> = keyword_parser::process_keywords(
        String::from("tests/test_keywords.txt")
    ).unwrap();

    for entry in keywords {
        let (key, score) = entry;
        println!("{}: {}", key, score);
    }
}