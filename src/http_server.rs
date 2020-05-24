use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("Read stream: \r\n{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    match stream.write(render_response(200, "text/html", "<html><body>Hello world</body></html>").as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

pub fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

fn render_response(code: u16, content_type: &str, content: &str) -> String {
    let header_code: String = format!("HTTP/1.1 {}", code);
    let header_content_type: String = format!("Content-Type: {}; charset=UTF-8", content_type);
    let headers: Vec<&str> = vec![
        &*header_code,
        &*header_content_type,
        "\r\n",
        content,
        ""
    ];

    return headers.join("\r\n");
}