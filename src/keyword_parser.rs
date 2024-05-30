use std::fs::File;
use std::collections::HashMap;

use std::io::prelude::*;

pub fn process_keywords(path: String) -> std::io::Result<HashMap<String, u32>> {
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


/*
TODO:
- make words regex-able
- make template doc for non-coders
*/