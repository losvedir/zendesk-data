#![feature(plugin)]

extern crate csv;

#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

extern crate "rustc-serialize" as rustc_serialize;

use std::io::{File, BufferedReader};
use std::option::{Option};
use std::str::{FromStr};

#[derive(RustcEncodable)]
struct User {
    id: Option<i32>,
    email: Option<String>,
    created_at: Option<String>,
    details: Option<String>,
    external_id: Option<i32>,
    is_active: Option<bool>,
    last_login: Option<String>,
    name: Option<String>,
    notes: Option<String>,
    organization_id: Option<i32>,
    phone: Option<String>,
    updated_at: Option<String>,
    is_verified: Option<bool>
}

impl User {
    fn empty() -> User {
        User {
            id: None,
            email: None,
            created_at: None,
            details: None,
            external_id: None,
            is_active: None,
            last_login: None,
            name: None,
            notes: None,
            organization_id: None,
            phone: None,
            updated_at: None,
            is_verified: None,
        }
    }
}

fn main() {
    handle_users();
}

fn handle_users() {
    let mut xml_reader = BufferedReader::new(File::open(&Path::new("xml-data/users.xml")));
    let mut csv_writer = csv::Writer::from_file(&Path::new("users.csv"));

    let _ = csv_writer.encode(("id", "email", "created-at", "details", "external-id", "is-active", "last-login", "name",
        "organization-id", "phone", "updated-at", "is-verified"));

    let re_begin_user = regex!(r"<user>");
    let re_end_user = regex!(r"</user>");
    let re_id = regex!(r"<id.*?>(.*)</id>");
    let re_email = regex!(r"<email.*?>(.*)</email>");
    let re_created_at = regex!(r"<created-at.*?>(.*)</created-at>");
    let re_details = regex!(r"<details.*?>(.*)</details>");
    let re_external_id = regex!(r"<external-id.*?>(.*)</external-id>");
    let re_is_active = regex!(r"<is-active.*?>(.*)</is-active>");
    let re_last_login = regex!(r"<last-login.*?>(.*)</last-login>");
    let re_organization_id = regex!(r"<organization-id.*?>(.*)</organization-id>");
    let re_phone = regex!(r"<phone.*?>(.*)</phone>");
    let re_updated_at = regex!(r"<updated-at.*?>(.*)</updated-at>");
    let re_is_verified = regex!(r"<is-verified.*?>(.*)</is-verified>");

    let mut user = User::empty();

    for l in xml_reader.lines() {
        let a: String = l.unwrap();
        let line: &str = a.as_slice();

        if re_begin_user.is_match(line) {
            user = User::empty();
        } else if re_end_user.is_match(line) {
            csv_writer.encode(&user);
        } else if re_id.is_match(line) {
            user.id = first_capture_as_i32(re_id.captures(line));
        } else if re_email.is_match(line) {
            user.email = first_capture_as_string(re_email.captures(line));
        } else if re_created_at.is_match(line) {
            user.created_at = first_capture_as_string(re_created_at.captures(line));
        } else if re_details.is_match(line) {
            user.details = first_capture_as_string(re_details.captures(line));
        } else if re_external_id.is_match(line) {
            user.external_id = first_capture_as_i32(re_external_id.captures(line));
        } else if re_is_active.is_match(line) {
            user.is_active = first_capture_as_bool(re_is_active.captures(line));
        } else if re_last_login.is_match(line) {
            user.last_login = first_capture_as_string(re_last_login.captures(line));
        } else if re_organization_id.is_match(line) {
            user.organization_id = first_capture_as_i32(re_organization_id.captures(line));
        } else if re_phone.is_match(line) {
            user.phone = first_capture_as_string(re_phone.captures(line));
        } else if re_updated_at.is_match(line) {
            user.updated_at = first_capture_as_string(re_updated_at.captures(line));
        } else if re_is_verified.is_match(line) {
            user.is_verified = first_capture_as_bool(re_is_verified.captures(line));
        }
    }
    let _ = csv_writer.flush();
}


fn first_capture_as_string(caps_line: Option<regex::Captures>) -> Option<String> {
    match caps_line {
        Some(caps) => { caps.at(1).map(|cap| cap.to_string()) },
        None => { None }
    }
}

fn first_capture_as_i32(caps_line: Option<regex::Captures>) -> Option<i32> {
    match caps_line {
        Some(caps) => { caps.at(1).and_then(|cap| FromStr::from_str(cap)) },
        None => { None }
    }
}

fn first_capture_as_bool(caps_line: Option<regex::Captures>) -> Option<bool> {
    match caps_line {
        Some(caps) => { caps.at(1).map(|cap| cap == "true") },
        None => { None }
    }
}
