use std::net::TcpListener;
use std::io::{Read,Write};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080");
    match listener{
        Ok(listener) => {
            println!("Server is running on 127.0.0.1:8080");
            //incoming() returns an iterator that yields a new TcpStream for each incoming connection
            for stream in listener.incoming(){
                
                let mut stream = stream.unwrap();
                println!("Connection established!");
                //创建缓冲区：
                let mut buffer = [0; 512];//声明了一个可变的变量 buffer，它是一个包含 512 个元素的数组，每个元素的初始值都是 0。这个数组用作缓冲区，用于存储从流中读取的数据。
                //从流中读取数据到缓冲区，然后将缓冲区的数据写回到流中
                stream.read(&mut buffer).unwrap();
                stream.write(&buffer).unwrap();

        }
    }
        Err(e) => println!("Failed to bind,with error {e}"),
    }

}
