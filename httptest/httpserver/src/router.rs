
use super::handler::{Handler,PageNotFoundHandler,StaticPageHandler,WebServiceHandler};
use http::{httprequest,httpresponse::HttpResponse,httprequest::HttpRequest};
use std::io::prelude::*;
pub struct Router{

}

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    println!("'GET' request for path: {}", s); // 打印路径
                    let route: Vec<&str> = s.split("/").collect();
                    if route.len() > 1 { // 检查路径分割结果
                        match route[1] {
                            "api" => {
                                let resp: HttpResponse = WebServiceHandler::handle(&req);
                                if let Err(e) = resp.send_response(stream) {
                                    eprintln!("Failed to send response: {}", e);
                                }
                            }
                            _ => {
                                let resp: HttpResponse = StaticPageHandler::handle(&req);
                                if let Err(e) = resp.send_response(stream) {
                                    eprintln!("Failed to send response: {}", e);
                                }
                            }
                        }
                    } else {
                        println!("Path does not contain a second segment.");
                    }
                }
            },
            _ => {
                println!("Unhandled request");
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                if let Err(e) = resp.send_response(stream) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
        }
    }
}