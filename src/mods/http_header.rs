use core::str::Split;
use std::collections::HashMap;

/// An HTTP Request parser
/// An HTTP Requset is made up of three parts:
/// 1. a request line
/// 2. header fields
/// 3. body (if needed)
/// reference: https://www.ibm.com/docs/en/cics-ts/5.3?topic=protocol-http-requests
/// this structure covers the first two parts (so called header)
pub struct RequestHeader {
    path: String,
    method: String,
    version: String,
    header_fields: HashMap<String, String>,
}
impl RequestHeader {
    /// returns `None` if the string is not a valid http request header
    pub fn new(string: String) -> Option<Self> {
        // println!("{}", &original_string);
        let mut lines = string.split("\r\n"); // http headers are separeted by CRLFs
        let line_one = lines.nth(0);
        let line_one = line_one?;
        // dbg!(line_one);
        // request line (first line)
        let (method, path, version) = RequestHeader::parse_request_line(line_one);
        let header_fields = RequestHeader::parse_header_fields(&mut lines);
        if method == "" || path == "" || version == "" {
            None
        }else{
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
    fn parse_request_line(line_one: &str) -> (String, String, String) {
        let mut line_one_iter = line_one.split(" ");
        let method = line_one_iter.next().unwrap_or("").to_string();
        let path = line_one_iter.next().unwrap_or("").to_string(); //TODO remove query strings
        let version = line_one_iter.next().unwrap_or("").to_string();
        (method, path, version)
    }
    fn parse_header_fields(lines: &mut Split<&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        loop {
            let line = lines.next();
            match line {
                Some(pair) => match pair {
                    "" => break,
                    l => {
                        let mut split = l.split(":");
                        // dbg!(&split.collect::<Vec<_>>());

                        let k = split.next();
                        if k == None {
                            continue;
                        }
                        let k = k.unwrap().to_owned();
                        let v = split.next();
                        if v == None {
                            continue;
                        }
                        let v = v.unwrap().to_owned();

                        map.insert(k, v);
                    }
                },
                None => break,
            }
        }
        map
    }
}
