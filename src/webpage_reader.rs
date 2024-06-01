use core::str::Split;

pub fn get_course_links() -> Result<Vec<String>, ureq::Error> {
    let body: String = ureq::get("https://www.pugetsound.edu/university-bulletin")
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    let mut out: Vec<String> = Vec::new();
    let mut split: Split<char> = body.split('\n');
    let mut flag_outer: bool = false;
    let mut flag_inner: bool = false;

    loop {
        let line = match split.next() {
            Some(res) => res,
            None => break,
        };

        /*
        This code is designed very specifically to scrub a specific piece
        of the UPS bulletin page. Since that page will change,
        this section of code will need to be rewritten so that we pull the
        necessary links. 

        The links we're looking for look like this:
        "https://www.pugetsound.edu/academics/*DEPARTMENT*/courses"
        */
        if line.trim() == "<span id=\"courses\"></span>" { flag_outer = true; }
        if line.trim().contains("<li>") && flag_outer {
            flag_inner = true;
            let mut pieces = line.split('\"');
            pieces.next();
            out.push(pieces.next().unwrap().trim().to_string());
        } else if !line.trim().contains("li") && !line.trim().contains("ul") && flag_inner {
            flag_inner = false;
            flag_outer = false;
        }
    }

    Ok(out)
}