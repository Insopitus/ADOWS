use adows::mods::http_header::RequestHeader;


#[test]
fn empty_string_test() {
    let header = RequestHeader::new(String::new());
    assert!(header.is_none());
}

// #[test]
// fn bad_request_line(){
//   let string = String::from("HTTP/1.1 GET A");
//   assert!(RequestHeader::new(string).is_none());
// }
