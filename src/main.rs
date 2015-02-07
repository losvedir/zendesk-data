#![feature(core)]
#![feature(path)]
#![feature(plugin)]
#![feature(io)]

extern crate csv;
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;
extern crate "rustc-serialize" as rustc_serialize;
extern crate xml;

use std::old_io::{File, BufferedReader};
use std::option::{Option};
use xml::reader::EventReader;
use xml::reader::events::*;

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

#[derive(RustcEncodable)]
struct Ticket {
    assigned_at: Option<String>,
    assignee_id: Option<String>,
    base_score: Option<String>,
    created_at: Option<String>,
    current_tags: Option<String>,
    description: Option<String>,
    due_date: Option<String>,
    entry_id: Option<String>,
    external_id: Option<String>,
    group_id: Option<String>,
    initially_assigned_at: Option<String>,
    latest_recipients: Option<String>,
    nice_id: Option<String>,
    organization_id: Option<String>,
    original_recipient_address: Option<String>,
    priority_id: Option<String>,
    recipient: Option<String>,
    requester_id: Option<String>,
    resolution_time: Option<String>,
    solved_at: Option<String>,
    status_id: Option<String>,
    status_updated_at: Option<String>,
    subject: Option<String>,
    submitter_id: Option<String>,
    ticket_type_id: Option<String>,
    updated_at: Option<String>,
    updated_by_type_id: Option<String>,
    via_id: Option<String>,
    score: Option<String>,
    problem_id: Option<String>,
    has_incidents: Option<String>
}

impl Ticket {
    fn empty() -> Ticket {
        Ticket {
            assigned_at: None,
            assignee_id: None,
            base_score: None,
            created_at: None,
            current_tags: None,
            description: None,
            due_date: None,
            entry_id: None,
            external_id: None,
            group_id: None,
            initially_assigned_at: None,
            latest_recipients: None,
            nice_id: None,
            organization_id: None,
            original_recipient_address: None,
            priority_id: None,
            recipient: None,
            requester_id: None,
            resolution_time: None,
            solved_at: None,
            status_id: None,
            status_updated_at: None,
            subject: None,
            submitter_id: None,
            ticket_type_id: None,
            updated_at: None,
            updated_by_type_id: None,
            via_id: None,
            score: None,
            problem_id: None,
            has_incidents: None
        }
    }
}

enum TicketField {
    AssignedAt,
    AssigneeId,
    BaseScore,
    CreatedAt,
    CurrentTags,
    Description,
    DueDate,
    EntryId,
    ExternalId,
    GroupId,
    InitiallyAssignedAt,
    LatestRecipients,
    NiceId,
    OrganizationId,
    OriginalRecipientAddress,
    PriorityId,
    Recipient,
    RequesterId,
    ResolutionTime,
    SolvedAt,
    StatusId,
    StatusUpdatedAt,
    Subject,
    SubmitterId,
    TicketTypeId,
    UpdatedAt,
    UpdatedByTypeId,
    ViaId,
    Score,
    ProblemId,
    HasIncidents,
}

fn main() {
    handle_users();
    handle_tickets();
}

fn handle_tickets() {
    let buf = BufferedReader::new(File::open(&Path::new("xml-data/tickets.xml")));
    let mut parser = EventReader::new(buf);
    let mut csv_writer = csv::Writer::from_file(&Path::new("tickets.csv"));
    let mut ticket = Ticket::empty();
    let mut current_tag: Option<TicketField> = None;
    let mut in_comments = false;

    // can't figure out how to output headers. The below won't work because tuples with
    // this many fields aren't auto-encodable.
    // let _ = csv_writer.encode(("assigned_at", "assignee_id", etc));

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes: _, namespace: _ } => {
                match &name.local_name[] {
                    "ticket" => { ticket = Ticket::empty(); },
                    "comments" => { in_comments = true; }
                    "assigned-at" => { current_tag = Some(TicketField::AssignedAt) },
                    "assignee-id" => { current_tag = Some(TicketField::AssigneeId) },
                    "base-score" => { current_tag = Some(TicketField::BaseScore) },
                    "created-at" => {
                        if !in_comments {
                            current_tag = Some(TicketField::CreatedAt);
                        }
                    },
                    "current-tags" => { current_tag = Some(TicketField::CurrentTags) },
                    "description" => { current_tag = Some(TicketField::Description) },
                    "due-date" => { current_tag = Some(TicketField::DueDate) },
                    "entry-id" => { current_tag = Some(TicketField::EntryId) },
                    "external-id" => { current_tag = Some(TicketField::ExternalId) },
                    "group-id" => { current_tag = Some(TicketField::GroupId) },
                    "initially-assigned-at" => { current_tag = Some(TicketField::InitiallyAssignedAt) },
                    "latest-recipients" => { current_tag = Some(TicketField::LatestRecipients) },
                    "nice-id" => { current_tag = Some(TicketField::NiceId) },
                    "organization-id" => { current_tag = Some(TicketField::OrganizationId) },
                    "original-recipient-address" => { current_tag = Some(TicketField::OriginalRecipientAddress) },
                    "priority-id" => { current_tag = Some(TicketField::PriorityId) },
                    "recipient" => { current_tag = Some(TicketField::Recipient) },
                    "requester-id" => { current_tag = Some(TicketField::RequesterId) },
                    "resolution-time" => { current_tag = Some(TicketField::ResolutionTime) },
                    "solved-at" => { current_tag = Some(TicketField::SolvedAt) },
                    "status-id" => { current_tag = Some(TicketField::StatusId) },
                    "status-updated-at" => { current_tag = Some(TicketField::StatusUpdatedAt) },
                    "subject" => { current_tag = Some(TicketField::Subject) },
                    "submitter-id" => { current_tag = Some(TicketField::SubmitterId) },
                    "ticket-type-id" => { current_tag = Some(TicketField::TicketTypeId) },
                    "updated-at" => { current_tag = Some(TicketField::UpdatedAt) },
                    "updated-by-type-id" => { current_tag = Some(TicketField::UpdatedByTypeId) },
                    "via-id" => {
                        if !in_comments {
                            current_tag = Some(TicketField::ViaId);
                        }
                    },
                    "score" => { current_tag = Some(TicketField::Score) },
                    "problem-id" => { current_tag = Some(TicketField::ProblemId) },
                    "has-incidents" => { current_tag = Some(TicketField::HasIncidents) },
                    _ => { current_tag = None; }
                }
            },
            XmlEvent::EndElement { name } => {
                if &name.local_name[] == "ticket" {
                    let _ = csv_writer.encode(&ticket);
                } else if &name.local_name[] == "comments" {
                    in_comments = false;
                }
                current_tag = None;
            },
            XmlEvent::Characters(text) => {
                match current_tag {
                    Some(TicketField::AssignedAt) => { ticket.assigned_at = Some(text) },
                    Some(TicketField::AssigneeId) => { ticket.assignee_id = Some(text) },
                    Some(TicketField::BaseScore) => { ticket.base_score = Some(text) },
                    Some(TicketField::CreatedAt) => { ticket.created_at = Some(text) },
                    Some(TicketField::CurrentTags) => { ticket.current_tags = Some(text) },
                    Some(TicketField::Description) => { ticket.description = Some(text) },
                    Some(TicketField::DueDate) => { ticket.due_date = Some(text) },
                    Some(TicketField::EntryId) => { ticket.entry_id = Some(text) },
                    Some(TicketField::ExternalId) => { ticket.external_id = Some(text) },
                    Some(TicketField::GroupId) => { ticket.group_id = Some(text) },
                    Some(TicketField::InitiallyAssignedAt) => { ticket.initially_assigned_at = Some(text) },
                    Some(TicketField::LatestRecipients) => { ticket.latest_recipients = Some(text) },
                    Some(TicketField::NiceId) => { ticket.nice_id = Some(text) },
                    Some(TicketField::OrganizationId) => { ticket.organization_id = Some(text) },
                    Some(TicketField::OriginalRecipientAddress) => { ticket.original_recipient_address = Some(text) },
                    Some(TicketField::PriorityId) => { ticket.priority_id = Some(text) },
                    Some(TicketField::Recipient) => { ticket.recipient = Some(text) },
                    Some(TicketField::RequesterId) => { ticket.requester_id = Some(text) },
                    Some(TicketField::ResolutionTime) => { ticket.resolution_time = Some(text) },
                    Some(TicketField::SolvedAt) => { ticket.solved_at = Some(text) },
                    Some(TicketField::StatusId) => { ticket.status_id = Some(text) },
                    Some(TicketField::StatusUpdatedAt) => { ticket.status_updated_at = Some(text) },
                    Some(TicketField::Subject) => { ticket.subject = Some(text) },
                    Some(TicketField::SubmitterId) => { ticket.submitter_id = Some(text) },
                    Some(TicketField::TicketTypeId) => { ticket.ticket_type_id = Some(text) },
                    Some(TicketField::UpdatedAt) => { ticket.updated_at = Some(text) },
                    Some(TicketField::UpdatedByTypeId) => { ticket.updated_by_type_id = Some(text) },
                    Some(TicketField::ViaId) => { ticket.via_id = Some(text) },
                    Some(TicketField::Score) => { ticket.score = Some(text) },
                    Some(TicketField::ProblemId) => { ticket.problem_id = Some(text) },
                    Some(TicketField::HasIncidents) => { ticket.has_incidents = Some(text) },
                    None => {}
                }
            },
            _ => {}
        }
    }
    let _ = csv_writer.flush();
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
        let line: &str = &a[];

        if re_begin_user.is_match(line) {
            user = User::empty();
        } else if re_end_user.is_match(line) {
            let _ = csv_writer.encode(&user);
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
        Some(caps) => {
            caps.at(1).and_then(|cap|
                match cap.parse() {
                    Ok(i) => { Some(i) },
                    _ => { None }
                }
            )
        },
        None => { None }
    }
}

fn first_capture_as_bool(caps_line: Option<regex::Captures>) -> Option<bool> {
    match caps_line {
        Some(caps) => { caps.at(1).map(|cap| cap == "true") },
        None => { None }
    }
}
