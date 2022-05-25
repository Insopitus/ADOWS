use super::HeaderFields;
use crate::utils::percent_decode;

/// An HTTP Request parser
///
/// An HTTP Requset is made up of three parts:
///
/// 1. a request line
/// 2. header fields
/// 3. body (if needed)
///
/// reference: https://www.ibm.com/docs/en/cics-ts/5.3?topic=protocol-http-requests
///
/// this structure covers the first two parts (so-called header)
pub struct RequestHeader {
    path: String,
    method: String,
    version: String,
    header_fields: HeaderFields,
}
impl RequestHeader {
    /// returns `None` if the string is not a valid http request header
    pub fn new(string: String) -> Option<Self> {
        // println!("{}", &original_string);
        let mut lines = string.split("\r\n"); // http headers are separeted by CRLFs
        let line_one = lines.next();
        let line_one = line_one?;
        // dbg!(line_one);
        // request line (first line)
        let (method, path, version) = RequestHeader::parse_request_line(line_one);
        let header_fields = HeaderFields::from(string.as_str());
        if method.is_empty() || path.is_empty() || version.is_empty() {
            None
        } else {
            Some(RequestHeader {
                path,
                method,
                version,
                header_fields,
            })
        }
    }
    pub fn get_method(&self) -> &String {
        &self.method
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }
    pub fn get_version(&self) -> &String {
        &self.version
    }
    pub fn is_get(&self) -> bool {
        self.method == "GET"
    }
    pub fn get_content_length(&self) -> usize {
        self.header_fields
            .get("Content-Length")
            .unwrap_or(&String::from("0"))
            .parse::<usize>()
            .unwrap_or(0) // TODO maybe shouldn't return 0 if parse failed
    }
    pub fn get_entity_tag(&self) -> Option<String> {
        self.header_fields
            .get("If-None-Match")
            .map(|s| s.trim().to_string())
    }
    fn parse_request_line(line_one: &str) -> (String, String, String) {
        let mut line_one_iter = line_one.split(' ');
        let method = line_one_iter.next().unwrap_or("").to_string();
        let path = line_one_iter
            .next()
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("")
            .to_string(); // remove query strings
        let path = percent_decode(&path);
        let version = line_one_iter.next().unwrap_or("").to_string();
        (method, path, version)
    }
}

#[cfg(test)]
mod test {
    use super::RequestHeader;
    #[test]
    fn standard_request_line() {
        let string = String::from("GET /index.html HTTP/1.1");
        assert!(RequestHeader::new(string).is_some());
    }

    #[test]
    fn empty_string() {
        let header = RequestHeader::new(String::new());
        assert!(header.is_none());
    }

    #[test]
    fn bad_request_line() {
        let string = String::from("HTTP/1.1 GET");
        assert!(RequestHeader::new(string).is_none());
    }
    // #[test]
    // fn bad_request_line_2(){
    //   let string = String::from("HTTP/1.1 GET A");
    //   assert!(RequestHeader::new(string).is_none());
    // }

    #[test]
    fn query_strings() {
        let string = String::from("GET /script/m.js?v=1 HTTP/1.1");
        let header = RequestHeader::new(string);
        assert!(header.is_some());
        let header = header.unwrap();
        let path = header.get_path();
        assert_eq!(path, "/script/m.js");
    }

    #[test]
    fn uri_encoded() {
        let string = String::from("GET /script/%E4%B8%AD%E6%96%87.js HTTP/1.1");
        let header = RequestHeader::new(string);
        assert!(header.is_some());
        let header = header.unwrap();
        let path = header.get_path();
        assert_eq!(path, "/script/中文.js");
    }
}
