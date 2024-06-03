pub mod keyword_parser;
pub mod webpage_reader;

fn main() {
    /*
    let bulletin_link = "https://www.pugetsound.edu/university-bulletin";
    let links = match webpage_reader::course_scores(bulletin_link) {
        Ok(res) => res,
        Err(e) => {
            println!("Encountered error! {:?}", e);
            return;
        },
    };
    for ((course, _), score) in links {
        println!("{}: {}", course, score);
    }

    let link: &str = "https://www.pugetsound.edu/academics/hispanic-studies/courses";
    let out = webpage_reader::dept_courses(link).unwrap();

    for (name, info) in out {
        println!("Course title: {}\nCourse description: {}\n", name, info);
    }

    let txt = String::from(" Here is my freaky chunk of text. humans EnvIronmentalism");
    let keywords = keyword_parser::process_keywords("tests/test_keywords.txt").unwrap();

    println!("{}", keyword_parser::score_text(txt, &keywords))
    */

    let out = webpage_reader::course_scores("https://www.pugetsound.edu/university-bulletin", "tests/test_keywords.txt").unwrap();

    for ((title, _), score) in out {
        if score == 0 { continue; }
        println!("{}: {}", title, score);
    }
}