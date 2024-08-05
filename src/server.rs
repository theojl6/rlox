use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = Rc::new(BufReader::new(&mut stream));
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:?}", http_request);
    let request_body: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take(2)
        .collect();
    println!("{:?}", request_body);

    let status_line = "HTTP/1.1 200 OK";

    let content = "Hello".to_string();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
