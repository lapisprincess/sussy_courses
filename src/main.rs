pub mod keyword_parser;
pub mod webpage_reader;

fn main() {
    let out = webpage_reader::course_scores("https://www.pugetsound.edu/university-bulletin", "tests/test_keywords.txt").unwrap();

    let mut hash_vec: Vec<(&(String, String), &u32)> = out.iter().collect();
    hash_vec.sort_by(|a, b| a.1.cmp(b.1));
    for ((title, _), score) in hash_vec {
        println!("{}: {}", title, score);
    }
}
