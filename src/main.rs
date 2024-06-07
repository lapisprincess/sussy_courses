use std::{env, path};

use slint;

pub mod keyword_parser;
pub mod webpage_reader;


const BULLETIN_LINK: &str = "https://www.pugetsound.edu/university-bulletin";
const APP_ID: &str = "org.gtk_rs.CourseRater";


slint::include_modules!();
fn main() {
    UI::new().unwrap().run().unwrap();

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

    let out = match webpage_reader::course_scores(BULLETIN_LINK, keywords_path) {
        Ok(res) => res,
        Err(_) => {
            println!("Invalid link to university bulletin! ({})", BULLETIN_LINK);
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