use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for possible_stream in listener.incoming() {
        let Ok(stream) = possible_stream else {
            continue;
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut client: TcpStream) {
    let buf_reader = BufReader::new(&client);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent length: {}\r\n\r\n{}",
        status_line, length, contents
    );
    client.write_all(response.as_bytes()).unwrap();
}
