use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, PartialEq,Clone)]
pub struct HttpResponse<'a> {
    pub version : &'a str,
    pub status_code: u16,
    pub status_text: &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<String> ,
}
impl<'a> Default for HttpResponse<'a>{
    fn default() -> Self {
        Self{
            version: "HTTP/1.1",
            status_code: 200,
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}
//定义了一个从 HttpResponse 类型到 String 类型的转换，将 HttpResponse 类型的实例转换成一个 String
impl<'a> From<HttpResponse<'a>> for String{
    fn from(res: HttpResponse<'a>) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code:u16,
        headers:Option<HashMap<&'a str,&'a str>>,
        body:Option<String>
    )-> HttpResponse{
        let mut response = HttpResponse::default();
        if status_code != 200{
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
        };

        response.status_text = match response.status_code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };
        response.body = body;
        response
            
    }
    //函数参数是一个对实现了Write特性的类型的可变引用，可以传入任何实现了Write特性的类型的实例
    pub fn send_response(&self,write_stream:&mut impl Write)->Result<(),io::Error>{
        let res = self.clone();
        let response_string :String = String::from(res);
        let _ = write!(write_stream,"{}",response_string)?;
        Ok(())
    }
    fn version(&self)->&str{
        self.version
    }
    fn status_code(&self)-> u16{
        self.status_code
    }
    fn status_text(&self)->&str{
        self.status_text
    }
    fn headers(&self)->String{
        let map: HashMap<&str,&str> = self.headers.clone().unwrap();
        let mut header_string:String = "".into();
        for (k,v) in map.iter(){
            header_string = format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string
    }
    pub fn body(&self) -> &str{
        match &self.body{
            Some(b)=>b.as_str(),
            None=>"",
        }
    }
        
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(200, None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 200,
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(404, None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 404,
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: 404,
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx"
                .to_string();
        assert_eq!(http_string, actual_string);
    }
}