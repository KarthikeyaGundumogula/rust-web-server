use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            println!("creating new thread");
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, html_res) = match &req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "home.html"),
        "GET /wait HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            println!("handling wait");
            ("HTTP/1.1 200 OK", "home.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "error.html"),
    };
    let contents = fs::read_to_string(html_res).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
