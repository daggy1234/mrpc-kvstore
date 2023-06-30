//! This code is a simple Rust frontend application that listens for incoming connections,
//! send a RPC request to the echo server, and sends a response back to the client.

pub mod kvstore {
    // The string specified here must match the proto package name
    mrpc::include_proto!("kvstore");
}

use kvstore::kvstore_client::KvstoreClient;
use kvstore::GetParams;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use shm::string::String;

// The main function that starts the server and listens for incoming connections.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // Loop over incoming connections.
    for stream in listener.incoming() {
        let stream = stream?;

        // Spawn a new thread to handle the connection.
        thread::spawn(|| {
            handle_connection(stream).unwrap_or_else(|error| {
                eprintln!("Error handling connection: {}", error);
            });
        });
    }
    Ok(())
}

// Function to handle the connection, read the request, and send the response.
fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_str = std::string::String::new();
    buf_reader.read_line(&mut request_str)?;

    // Parse the request string and extract the URI.
    let request_parts: Vec<&str> = request_str.trim().split_whitespace().collect();
    let uri = request_parts[1];

    // Connect to the Greeter service and send the HelloRequest.
    let client = KvstoreClient::connect("localhost:5000")?;
    let req = GetParams {
        key: String::from("hello!"),
    };
    let reply = smol::block_on(client.get(req))?;
    println!("reply: {}", String::from(&reply.key));

    // Prepare and send the HTTP response.
    let status_line = "HTTP/1.1 200 OK";
    let content = String::from(&reply.key) + "\n";
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
