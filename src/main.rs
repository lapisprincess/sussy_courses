use std::{env, path};

pub mod keyword_parser;
pub mod webpage_reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <path to keywords file> (dept codes)");
        return;
    }

    let keywords_path: &String = &args[1];
    if !path::Path::new(keywords_path).exists() {
        println!("Keywords path ({}) doesn't exist!", keywords_path);
        return;
    }

    let dept_codes: &[String] = &args[2..];

    let bulletin_link: String = String::from("https://www.pugetsound.edu/university-bulletin");
    let out = match webpage_reader::course_scores(&bulletin_link, keywords_path) {
        Ok(res) => res,
        Err(_) => {
            println!("Invalid link to university bulletin! ({})", bulletin_link);
            return;
        }
    };

    let mut valid: bool;
    let mut hash_vec: Vec<(&(String, String), &u32)> = out.iter().collect();
    hash_vec.sort_by(|a, b| a.1.cmp(b.1));
    for ((title, _), score) in hash_vec {
        valid = false;
        for dept in dept_codes {
            if title.contains(dept) { valid = true; }
        }
        if !valid { continue; }
        println!("{}: {}", title, score);
    }
}
