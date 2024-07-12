use super::router::Router;
use http::httprequest::HttpRequest;
use std::net::TcpListener;
use std::str;
use std::io::Read;
pub struct Server<'a>{
    socket_addr: &'a str,
}
impl<'a> Server<'a>{
    pub fn new(socket_addr: &'a str)->Self{
        Server{socket_addr}
}

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connected established");
        
            let mut read_buffer = Vec::new(); // 使用Vec<u8>作为动态数组
            stream.read_to_end(&mut read_buffer).unwrap(); // 读取流中的所有数据到动态数组中

            // 打印buffer信息
            //println!("Buffer content: {:?}", String::from_utf8_lossy(&read_buffer));

            let req: HttpRequest = String::from_utf8(read_buffer).unwrap().into();
            Router::route(req, &mut stream);
            
    }
}
}
