pub mod keyword_parser;
pub mod webpage_reader;

/*
fn get_course_data() -> Result<HashMap<String, String, u16>, ureq::Error> {

    Ok()
}
*/


fn main() {
    let links = match webpage_reader::get_course_links() {
        Ok(res) => res,
        Err(e) => {
            println!("Encountered error! {:?}", e);
            return;
        },
    };
    for link in links {
        println!("{}", link);
    }
}
