use std::io::{Read,Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connection established!");
    let sent_message = "Hello, server!";
    stream.write(sent_message.as_bytes()).unwrap();
    let mut buffer = [0; 12];
    stream.read(&mut buffer).unwrap();
    println!("Response from server: {:?}", str::from_utf8(&buffer).unwrap());


}
