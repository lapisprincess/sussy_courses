use lopdf::Document;

fn parse_pdf() {
    let doc = match Document::load("bulletin_example.pdf") {
        Ok(d) => d,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };
    /* 
    for (page, id) in doc.get_pages() {
        if page == 143 {
            let contents = doc.get_page_content(id);
            println!("{}", Document::decode_text(None, contents.unwrap()));
        }
    }
    */
    let mut pages: Vec<u32> = (170..180).collect();
    print_lines_from_page(doc, pages, 5);
}

fn print_lines_from_page(doc: Document, pages: Vec<u32>, num_lines: u32) {
    for page in pages {
        let content = doc.extract_text(&[page]).unwrap();
        let mut split = content.split('\n');
        for _ in 0..num_lines { println!("{}", split.next().unwrap()); }
        println!("==============");
    }
}

