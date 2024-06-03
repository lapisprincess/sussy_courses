use std::collections::HashMap;
use sussy_courses::keyword_parser;

#[test]
fn keywords_accurate() {
    let keywords: HashMap<String, u32> = keyword_parser::process_keywords(
        "tests/test_keywords.txt"
    ).unwrap();

    assert_eq!(10, *keywords.get("environmentalism").unwrap());
    assert_eq!(5, *keywords.get("environment").unwrap());
    assert_eq!(1, *keywords.get("globe").unwrap());
    assert_eq!(1, *keywords.get("warming").unwrap());
    assert_eq!(1, *keywords.get("global").unwrap());
    assert_eq!(5, *keywords.get("green").unwrap());
    assert_eq!(5, *keywords.get("humans").unwrap());
    assert_eq!(10, *keywords.get("global warming").unwrap());
    assert_eq!(10, *keywords.get("sustainability").unwrap());
}