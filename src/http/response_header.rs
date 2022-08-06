use std::fmt::Display;

use super::HeaderFields;

pub struct ResponseHeader {
    pub code: u32,
    header_fields: HeaderFields,
}
impl ResponseHeader {
    pub fn new(code: u32) -> Self {
        let header_fields = HeaderFields::new();
        ResponseHeader {
            code,
            header_fields,
        }
    }
    pub fn insert_field(&mut self, k: String, v: String) {
        self.header_fields.insert(k, v);
    }
    /// create response line from code
    fn get_response_line(&self) -> String {
        let mut response_line = String::from("HTTP/1.1");
        response_line.push(' ');
        let code_desc = match self.code {
            200 => "OK",
            304 => "NOT MODIFIED",
            400 => "BAD REQUEST",
            404 => "NOT FOUND",
            _ => "BAD REQUEST",
        };
        response_line.push_str(self.code.to_string().as_str());
        response_line.push(' ');
        response_line.push_str(code_desc);
        response_line
    }
}
impl Display for ResponseHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response_line = self.get_response_line();
        write!(
            f,
            "{}\r\n{}\r\n",
            response_line,
            self.header_fields
        )
    }
}

#[cfg(test)]
mod test {
    use super::ResponseHeader;

    #[test]
    fn simple_200_response() {
        let response_header = ResponseHeader::new(200);
        assert_eq!(response_header.to_string(), "HTTP/1.1 200 OK\r\n\r\n");
    }
}
