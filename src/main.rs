use std::collections::HashMap;

use lopdf::Document;

pub mod keyword_parser;


fn parse_pdf() {
    let doc = match Document::load("bulletin_example.pdf") {
        Ok(d) => d,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };
    println!("{}", doc.extract_text(&[231]).unwrap());
}

fn main() {
    let keywords: HashMap<String, u32> = match keyword_parser::process_keywords(
        String::from("tests/test_keywords.txt")
    ) {
        Ok(k) => k,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };

    for entry in keywords {
        let (key, score) = entry;
        println!("{}: {}", key, score);
    }

    parse_pdf();
}