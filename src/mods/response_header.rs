use super::header_fields::HeaderFields;

pub struct ResponseHeader {
    response_line: String,
    header_fields: HeaderFields,
}
impl ResponseHeader {
    pub fn new(code: u32) -> Self {
        let mut response_line = String::from("HTTP/1.1");
        response_line.push_str(" ");
        let code_desc = match code {
            200 => "OK",
            304 => "NOT MODIFIED",
            400 => "BAD REQUEST",
            404 => "NOT FOUND",
            _ => "BAD REQUEST",
        };
        response_line.push_str(&format!("{} {}", &code.to_string(), code_desc));

        let header_fields = HeaderFields::new();
        
        ResponseHeader {
            response_line,
            header_fields,
        }
    }
    pub fn insert_field(&mut self,k:String,v:String){
        self.header_fields.insert(k, v);
    }
    /// make a valid valid http header string
    pub fn to_string(&self) -> String {
        format!("{}\r\n{}\r\n",self.response_line,self.header_fields.to_string())
    }
}


#[cfg(test)]
mod test {
    use super::ResponseHeader;

  #[test]
  fn simple_200_response(){
    let response_header = ResponseHeader::new(200);
    assert_eq!(response_header.to_string(),"HTTP/1.1 200 OK\r\n\r\n");
  }
}