use std::collections::HashMap;

use lazy_static::lazy_static;
use tokio::{
    net::{TcpListener, TcpStream},
    prelude::*,
};

const GET_HEADER: &[u8] = b"GET / HTTP/1.1\r\n";
const OK_RESPONSE_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE_HEADER: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

lazy_static! {
    static ref RESPONSE_MAP: HashMap<&'static str, String> = [
        (
            OK_RESPONSE_HEADER,
            std::fs::read_to_string("hello.html").unwrap()
        ),
        (
            NOT_FOUND_RESPONSE_HEADER,
            std::fs::read_to_string("404.html").unwrap()
        )
    ]
    .iter()
    .cloned()
    .collect();
}

fn main() {
    let addr = "127.0.0.1:7878".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            let connection = handle_connection_async(socket).then(|result| {
                println!("handled the connection; success={:?}", result.is_ok());
                Ok(())
            });

            tokio::spawn(connection);
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    tokio::run(server);
}

fn handle_connection_async(stream: TcpStream) -> impl Future {
    let buffer = vec![0u8; 512];
    tokio::io::read(stream, buffer).and_then(|(socket, buffer, _)| {
        let header = if buffer.starts_with(GET_HEADER) {
            OK_RESPONSE_HEADER
        } else {
            NOT_FOUND_RESPONSE_HEADER
        };
        let contents = RESPONSE_MAP.get(header).unwrap();
        let response = format!("{}{}", header, contents);

        tokio::io::write_all(socket, response)
    })
}
