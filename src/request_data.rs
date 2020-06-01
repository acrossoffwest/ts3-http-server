extern crate regex;

pub struct RequestData {
    method: String,
    content: String
}

pub struct HeaderItem {
    name: String,
    value: String
}

pub fn build_request(request_content: String) -> RequestData {
    println!("'{}'", request_content);
    let mut contents: Vec<String> = split_request_content(request_content);
    let mut headers: Vec<HeaderItem> = Vec::new();

    return RequestData {
        method: String::from("GET"),
        content: String::from("test")
    };
}

fn split_request_content(request_content: String) -> Vec<String> {
    let regular = regex::Regex::new(r"\r\n\r\n").unwrap();
    let mut contents: Vec<String> = Vec::new();

    for part in regular.split(request_content.as_str()) {
        contents.push(String::from(part));
    }

    contents
}

fn split_headers(headers_string: String) -> Vec<HeaderItem> {
    let regular = regex::Regex::new(r"\r\n").unwrap();
    let mut headers: Vec<String> = Vec::new();

    for part in regular.split(headers_string.as_str()) {
        headers.push(String::from(part));
    }

    contents
}

fn handle_header_line(header_line: String) -> HeaderItem {
    let regular = regex::Regex::new(r"\r\n").unwrap();
    let mut headers: Vec<String> = Vec::new();

    for part in regular.split(headers_string.as_str()) {
        headers.push(String::from(part));
    }

    contents
}