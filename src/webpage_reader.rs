use core::str::Split;
use std::collections::HashMap;

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
// pulls (Title, Description) for every course
pub fn dept_courses(dept_link: &str) -> Result<Vec<(String, String)>, ureq::Error> {
    let mut out: Vec<(String, String)> = Vec::new();

    let mut course_title: Option<String> = None;
    let mut course_descr: Option<String> = None;

    let mut body_iter: Split<char>;
    let mut left_trim: Split<char>;
    let mut right_trim: Split<char>;

    let body: String = ureq::get(dept_link)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    body_iter = body.split('\n');

    let mut line: &str;
    loop {
        line = match body_iter.next() {
            Some(res) => res,
            None => break,
        };

        // title located inside "accordion-title" div
        if line.trim().contains("accordion-title") {
            left_trim = line.split('>');
            left_trim.next();
            left_trim.next();
            right_trim = left_trim
                .next().unwrap()
                .split('<');
            course_title = Some(right_trim.next().unwrap().to_string());
        }

        // paragraph located just under "card-body" div
        if line.trim().contains("card-body") {
            left_trim = body_iter
                .next().unwrap()
                .split('>');
            left_trim.next();
            right_trim = left_trim
                .next().unwrap()
                .split('<');
            course_descr = Some(right_trim.next().unwrap().to_string());
        }

        // match titles with descriptions
        if course_title != None && course_descr != None {
            out.push((course_title.unwrap(), course_descr.unwrap()));
            course_title = None;
            course_descr = None;
        }
    }

    Ok(out)
}


pub fn course_scores(bulletin_link: &str) -> Result<HashMap<(String, String), u16>, ureq::Error> {
    let mut out: HashMap<(String, String), u16> = HashMap::new();
    let links: Vec<String> = course_links(bulletin_link).unwrap();

    for link in links {
        let info: Vec<(String, String)>;
        let body: String = ureq::get(&link)
            .set("Example-Header", "header value")
            .call()?
            .into_string()?;
        let mut body_iter = body.split('\n');
        loop {

            let line = match body_iter.next() {
                Some(res) => res,
                None => break,
            };

        }

    }

    Ok(out)
}

