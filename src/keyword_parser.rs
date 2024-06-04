use core::str::Split;

use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Result;


// Given a path to a list of keywords with affiliated scores
// (formatted correctly),
// returns a hashmap of the keywords with corresponding scores
pub fn process_keywords(path: &str) -> Result<HashMap<String, u32>> {
    let mut file: File = File::open(path)?;
    let mut contents: String = String::new();
    let mut out: HashMap<String, u32> = HashMap::new();

    file.read_to_string(&mut contents)?;
    let mut lines = contents.split('\n');

    loop {
        let line = match lines.next() {
            Some(txt) => txt,
            None => break,
        };
        if line.len() <= 1 || line.as_bytes()[0] as char != '*' { continue; }

        let mut line_chars = line.chars();
        line_chars.next(); // get rid of star so we just have number
        let score: u32 = match line_chars.as_str().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let keywords = match lines.next() {
            Some(txt) => txt,
            None => break, // score's on last line of file
        };
        
        for word in keywords.split(';') { // second iter loop
            out.insert(word.trim().to_string(), score);
        }
    }

    Ok(out)
}


// Given a body of text and a hashmap of keywords and scores,
// returns a score of how often keywords appear in the text
pub fn score_text(txt: String, word_scores: &HashMap<String, u32>) -> u32 {
    let mut score: u32 = 0;

    for (search_word, search_score) in word_scores {
        if txt.to_lowercase().contains(search_word) { score += search_score; }
    }

    score
}
