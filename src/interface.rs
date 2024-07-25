use std::{env, fs};
use open;

use rfd::FileDialog;
use slint::{Model, SharedString};

use crate::webpage_reader;

slint::include_modules!();
pub fn init_interface_functionality() -> UI {
    let ui = UI::new().unwrap();


    // Add department code
    let ui_weak = ui.as_weak();
    ui.on_add_dept(move || {
        let mut departments: Vec<SharedString> = ui_weak
            .unwrap().get_department_codes()
            .iter().collect();
        departments.push("DEPT".into());
        let departments_model = std::rc::Rc::new(
            slint::VecModel::from(departments)
        );
        ui_weak.unwrap().set_department_codes(departments_model.into());
    });

    // Subtract department code
    let ui_weak = ui.as_weak();
    ui.on_sub_dept(move || {
        let mut departments: Vec<SharedString> = ui_weak
            .unwrap().get_department_codes()
            .iter().collect();
        departments.pop();
        let departments_model = std::rc::Rc::new(
            slint::VecModel::from(departments)
        );
        ui_weak.unwrap().set_department_codes(departments_model.into());
    });

    // Test URL
    ui.on_test_url(move |url| {
        let _ = open::that(url.as_str());
    });

    // Load keyword file
    ui.on_load_keywords(move || {
        let file_picker = FileDialog::new()
            .add_filter("text", &["txt"])
            .set_directory("data/")
            .pick_file();
        if let Some(path) = file_picker {
            let out = fs::read_to_string(path).unwrap().into();
            return out;
        } else {
            return "".into()
        }
    });

    // Run code TODO
    ui.on_run(move |url, keywords, narrow_by_dept, dept_codes| {
        let out = match webpage_reader::course_scores(&url, &keywords) {
            Ok(res) => res,
            Err(_) => {
                println!("Problemmmmmm");
                return;
            }
        };

        let mut valid: bool;
        let mut hash_vec: Vec<(&(String, String), &u32)> = out.iter().collect();
        hash_vec.sort_by(|a, b| a.1.cmp(b.1));

        for ((title, _), score) in hash_vec {
            // valid = false;
            // for dept in dept_codes {
            //     if title.contains(dept) { valid = true; }
            // }
            // if !valid { continue; }
            println!("{}: {}", title, score);
        }
    });

    ui
}