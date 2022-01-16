/// An HTTP Request parser
/// An HTTP Requset is made up of three parts:
/// 1. a request line
/// 2. header fields
/// 3. body (if needed)
/// reference: https://www.ibm.com/docs/en/cics-ts/5.3?topic=protocol-http-requests
/// this structure covers the first two part (so called header)
pub struct RequestHeader {
    orginal_string: String,
    path: String,
    method: String,
    version: String,
    content_length:usize,
}
impl RequestHeader {
    pub fn new(string: String) -> Self {
        let original_string = string.to_owned();
        println!("{}", &original_string);
        let line_one = string.split("\r\n").nth(0); // http headers are separeted by CRLFs
        let line_one = line_one.expect("Invalid HTTP Content.");
        dbg!(line_one);
        // request line (first line)
        let (method,path,version) = RequestHeader::parse_request_line(line_one);
      
        RequestHeader {
            orginal_string: original_string,
            path,
            method,
            version,
            content_length:64,
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

    fn parse_request_line(line_one:&str)->(String,String,String){
      let mut line_one_iter = line_one.split(" ");
      let method = line_one_iter
          .next()
          .expect("Invalid HTTP Content.")
          .to_string();
      let path = line_one_iter
          .next()
          .expect("Invalid HTTP Content.")
          .to_string(); //TODO remove query strings
      let version = line_one_iter
          .next()
          .expect("Invalid HTTP Content.")
          .to_string();
      (method,path,version)
    }
    fn parse_header_fields(){}
}
