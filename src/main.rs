#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

use std::io::{File, BufferedReader};

fn main() {
    let file = File::open(&Path::new("xml-data/users.xml"));
    let mut reader = BufferedReader::new(file);
    let re = regex!(r"<name>.*</name>");
    let mut names: i32 = 0;

    for line in reader.lines() {
        if re.is_match(line.unwrap().as_slice()) {
            names += 1;
        }
    }
    println!("{}", names);
}
