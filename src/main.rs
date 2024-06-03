pub mod keyword_parser;
pub mod webpage_reader;

/*
fn get_course_data() -> Result<HashMap<String, String, u16>, ureq::Error> {

    Ok()
}
*/


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
    */

    let link: &str = "https://www.pugetsound.edu/academics/hispanic-studies/courses";
    let out = webpage_reader::dept_courses(link).unwrap();

    for (name, info) in out {
        println!("Course title: {}\nCourse description: {}\n", name, info);
    }
}