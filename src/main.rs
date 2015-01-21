extern crate xml;

use std::io::{File, BufferedReader};
use xml::reader::EventReader;
use xml::reader::events::*;

fn main() {
    let file = File::open(&Path::new("xml-data/users.xml")).unwrap();
    let reader = BufferedReader::new(file);
    let mut parser = EventReader::new(reader);
    let mut in_name = true;
    let mut names : i32 = 0;

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes: _, namespace: _ } => {
                if name.local_name == "name" {
                    in_name = true;
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "name" {
                    in_name = false;
                }
            },
            XmlEvent::Characters(chars) => {
                if in_name {
                    // println!("{}", chars);
                    names += 1;
                }
            },
            _ => {}
        }
    }

    println!("{}", names);
}
