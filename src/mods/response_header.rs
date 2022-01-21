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
    pub fn to_string(&self) -> String {
        format!("{}\r\n{}\r\n",self.response_line,self.header_fields.to_string())
    }
}
