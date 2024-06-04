use core::str::Split;

use std::collections::HashMap;

use sussy_courses::keyword_parser::{process_keywords, score_text};


// Given the link to the school's bulletin,
// returns a vectory of links to every department's course page
pub fn course_links(bulletin_link: &str) -> Result<Vec<String>, ureq::Error> {
    let mut out: Vec<String> = Vec::new();
    let mut flag_outer: bool = false;
    let mut flag_inner: bool = false;
    
    let mut body_iter: Split<char>;
    let mut html_iter: Split<char>;
    let mut line: &str;

    let body: String = ureq::get(bulletin_link)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    body_iter = body.split('\n');

    loop {
        line = match body_iter.next() {
            Some(res) => res,
            None => break,
        };

        /*
        This code is designed very specifically to scrub a specific piece
        of the UPS bulletin page. Since that page will inevitably change,
        this section of code will need to be rewritten so that we pull the
        necessary links. 

        The links we're looking for look like this:
        "https://www.pugetsound.edu/academics/*DEPARTMENT*/courses"
        */
        if line.trim() == "<span id=\"courses\"></span>" { flag_outer = true; }
        if line.trim().contains("<li>") && flag_outer {
            flag_inner = true;
            html_iter = line.split('\"');
            html_iter.next();
            out.push(html_iter.next().unwrap().trim().to_string());
        } else if !line.trim().contains("li") && !line.trim().contains("ul") && flag_inner {
            flag_inner = false;
            flag_outer = false;
        }
    }

    Ok(out)
}



// Given a link to a department's course page, 
// returns a vectory of strings (Title, Description) for every course
pub fn dept_courses(dept_link: &str) -> Result<Vec<(String, String)>, ureq::Error> {
    let mut out: Vec<(String, String)> = Vec::new();

    let mut course_title: Option<String> = None;
    let mut course_descr: Option<String> = None;
    let mut course_caption: String;

    let mut body_iter: Split<char>;
    let mut left_trim: Split<char>;
    let mut right_trim: Split<char>;

    let body: String = ureq::get(dept_link)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    body_iter = body.split('\n');

    println!("{}", dept_link);

    let mut line: &str;
    loop {
        line = match body_iter.next() {
            Some(res) => res,
            None => break,
        };

        // title located inside "accordion-title" div
        if line.trim().contains("caption") {
            left_trim = line.split('>');
            left_trim.next();
            right_trim = left_trim.next().unwrap().split('<');

            course_caption = format!("({}) ", 
                right_trim.next().unwrap().to_string()
            );

            line = body_iter.next().unwrap();

            left_trim = line.split('>');
            left_trim.next();
            left_trim.next();
            right_trim = left_trim.next().unwrap().split('<');
            course_title = Some(course_caption + right_trim.next().unwrap());
            println!("{}", course_title.clone().unwrap());
        }

        // paragraph located just under "card-body" div
        if line.trim().contains("card-body") {
            line = body_iter.next().unwrap().trim();
            if !line.contains("<p>") { 
                course_descr = Some(String::from("(No description given)"));
            } else {
                left_trim = line.split('>');
                left_trim.next();
                right_trim = left_trim
                    .next().unwrap()
                    .split('<');
                course_descr = Some(right_trim.next().unwrap().to_string());
            }
        }

        // match titles with descriptions
        if course_title != None && course_descr != None {
            out.push((course_title.unwrap(), course_descr.unwrap()));
            course_title = None;
            course_descr = None;
        }
    }

    println!();
    Ok(out)
}


// Given a link to the school's bulletin and the path to a set of keywords,
// returns a hashmap containing a course's title, description, and score
pub fn course_scores(bulletin_link: &str, keyword_file_path: &str) -> Result<HashMap<(String, String), u32>, ureq::Error> {
    let mut out: HashMap<(String, String), u32> = HashMap::new();

    let word_scores: HashMap<String, u32>;
    let mut score: u32;

    let mut all_courses: Vec<(String, String)> = Vec::new();
    let links: Vec<String> = match course_links(bulletin_link) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    // gather all courses from all departments
    for link in links {
        all_courses.append(&mut dept_courses(&link).unwrap());
    }

    word_scores = match process_keywords(keyword_file_path) {
        Ok(res) => res,
        Err(_) => {
            println!("Invalid path for keyword scores!");
            std::process::exit(1);
        }
    };
    for (title, description) in all_courses {
        score = score_text(description.clone(), &word_scores);
        out.insert((title, description), score);
    }

    Ok(out)
}