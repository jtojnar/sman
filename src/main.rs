extern crate cursive;
extern crate glob;
extern crate regex;

use cursive::Cursive;
use cursive::traits::Scrollable;
use cursive::views::Dialog;
use cursive::views::SelectView;
use glob::glob;
use regex::Regex;
use std::env;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let re = Regex::new(r"^/usr/share/man/man[^/]+/([^.]+).([^.]+)").unwrap();
    for page in env::args().skip(1) {
        let mut section_select = SelectView::new();

        let pattern = format!("/usr/share/man/man*/{}.*", page);
        let paths = glob(pattern.as_str()).expect("Pattern error");
        for path in paths {
            match path {
                Ok(p) => {
                    let file = format!("{}", p.display());
                    let c = re.captures(file.as_str()).unwrap();
                    let page = format!("{}", c.at(1).unwrap());
                    let section = format!("{}", c.at(2).unwrap());
                    let label = format!("{} ({})", page, section);
                    section_select.add_item(label, (section, page));
                }
                // Inaccessible directories are simply ignored,
                // as man viewer probably cannot reach them either.
                Err(_) => {}
            };
        }

        section_select.set_on_submit(|s, &(ref section, ref page)| {
            s.quit();
            Command::new("man")
                .arg(section)
                .arg(page)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("failed to execute process");
        });

        let mut siv = Cursive::default();
        siv.add_layer(Dialog::around(section_select.scrollable()).title("Which section do you wish to open?"));
        siv.run();
    }
}
