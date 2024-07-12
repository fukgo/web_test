use std::collections::HashMap;

//PartialEq用于解析和自动化脚本里面作比较
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
#[derive(Debug,PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}
#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug,PartialEq)]
pub struct HttpRequest{
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String,String>,
    pub body: String,

}
fn parse_header_line(s:&str)->(String,String){
    let mut header_items = s.split(':');
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }
    (key, value)

}
fn parse_request_line(s:&str)->(Method,Resource,Version){
    let mut parts = s.split_whitespace();//按空格分割字符串
    let method = parts.next().unwrap().into();
    let resource = Resource::Path(parts.next().unwrap().to_string());
    let version = parts.next().unwrap().into();

    (method,resource,version)

}

impl From<String> for HttpRequest {
    fn from(send_request: String) -> Self {
        println!("{:?}",send_request);
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut body_lines = Vec::new(); // 使用Vec收集请求体的所有行

        for line in send_request.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = parse_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if !line.is_empty() {
                body_lines.push(line); // 收集请求体的行
            }
        }

        let parsed_body = body_lines.join("\n"); // 将请求体的所有行合并为一个字符串

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body,
        }
    }
}

//From<&str>用于将字符串转换为method类型
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
    
}
impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
        
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_read_http() {
        let s =
            String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n'");
        let mut expected_header: HashMap<String, String> = HashMap::new();
        expected_header.insert("Host".into(), "localhost".into());
        expected_header.insert("Accept".into(), "*/*".into());
        expected_header.insert("User-Agent".into(), "curl/7.71.1".into());

        println!("{:?}", expected_header);
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(expected_header, req.headers);
    }
}
