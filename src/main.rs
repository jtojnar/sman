extern crate cursive;
extern crate glob;
extern crate regex;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use cursive::Cursive;
use cursive::traits::Scrollable;
use cursive::views::Dialog;
use cursive::views::SelectView;
use glob::glob;
use regex::Regex;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::str;

fn main() {
    pretty_env_logger::init();

    let manpath = Command::new("man").arg("--path").output().expect("unable to get man path");

    let re = Regex::new(r"man[^/]+/([^.]+)\.([^.]+)").unwrap();
    for page in env::args().skip(1) {
        let mut section_select = SelectView::new();

        for manprefix in manpath.stdout.split(|c| *c == b':').map(|p| str::from_utf8(p).unwrap()) {
            debug!("Scanning {}", manprefix);
            let pattern = format!("{}/man*/{}.*", manprefix, page);
            let paths = glob(pattern.as_str()).expect("Pattern error");
            for path in paths {
                match path {
                    Ok(p) => {
                        debug!("Adding {}", p.display());
                        let file = format!("{}", p.display());
                        let c = re.captures(file.as_str()).unwrap();
                        let page = format!("{}", c.get(1).map_or("", |m| m.as_str()));
                        let section = format!("{}", c.get(2).map_or("", |m| m.as_str()));
                        let label = format!("{} ({})", page, section);
                        section_select.add_item(label, (section, page));
                    }
                    // Inaccessible directories are simply ignored,
                    // as man viewer probably cannot reach them either.
                    Err(_) => {}
                };
            }
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
