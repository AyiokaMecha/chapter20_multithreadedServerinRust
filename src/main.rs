use std::{fs, thread};
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
mod lib;


fn main() {
    use crate::lib;
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = lib::ThreadPool::new(5);
    loop {
        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();
            // thread::spawn(|| {
            //     handle_connection(stream);
            // });
            pool.execute(|| {
                handle_connection(stream);
            })
    }


        // handle_connection(stream);
        // println!("Connection established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> =buf_reader
    //     .lines()
    //     .map(|result|result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    //
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    // let (status_line, filename) =
    //     if request_line == "GET / HTTP/1.1" {
    //         ("HTTP/1.1 200 OK", "./src/staticAssets/hello.html")
    //     } else {
    //         ("HTTP/1.1 404 NOT FOUND", "./src/staticAssets/404.html")
    //     };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/staticAssets/hello.html"),
        "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./src/staticAssets/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./src/staticAssets/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

    // println!("Request: {:#?}", http_request);
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("./src/staticAssets/hello.html").unwrap();
    //     let length = contents.len();
    //     let response = format!(
    //         "{status_line}\r\n\
    //     Content-length: {length}\r\n\r\n\
    //     {contents}"
    //     );
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("./src/staticAssets/404.html").unwrap();
    //     let length = contents.len();
    //
    //     let response = format!(
    //         "{status_line}\r\n\
    //         Content-lenght: {length}\r\n\r\n{contents}"
    //     );
    // }
}

