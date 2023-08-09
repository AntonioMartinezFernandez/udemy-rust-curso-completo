use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    /*************************************
     *
     *          WEB SERVER
     *
     *************************************/

    // Init TCP listener (init server)
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(&address).unwrap();
    println!("server started at {}", address);

    // Listen connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Handle requests
fn handle_connection(mut stream: TcpStream) {
    println!("stream received from {}", stream.peer_addr().unwrap());

    // Create buffer
    let mut buffer = [0; 1024];

    // Read stream to buffer
    stream.read(&mut buffer).unwrap();

    // Print buffer
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    route_request(stream, buffer);
}

// Route Request
fn route_request(stream: TcpStream, buffer: [u8; 1024]) {
    let index_route = b"GET / HTTP/1.1"; // http://localhost:8000/
    let about_route = b"GET /about HTTP/1.1"; // http://localhost:8000/about

    if buffer.starts_with(index_route) {
        send_response(stream, "html/index.html");
    } else if buffer.starts_with(about_route) {
        send_response(stream, "html/about.html");
    } else {
        send_response(stream, "html/404.html");
    }
}

// Send Response
fn send_response(mut stream: TcpStream, file_path: &str) {
    // Read index.html file content
    let content = fs::read_to_string(file_path).unwrap();

    // Format the response with http code, content length and content (welcome back to the good old days...)
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
