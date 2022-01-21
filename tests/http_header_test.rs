use adows::mods::request_header::RequestHeader;

#[test]
fn standard_request_line(){
  let string = String::from("GET /index.html HTTP/1.1");
  assert!(RequestHeader::new(string).is_some());
}

#[test]
fn empty_string() {
    let header = RequestHeader::new(String::new());
    assert!(header.is_none());
}

#[test]
fn bad_request_line(){
  let string = String::from("HTTP/1.1 GET");
  assert!(RequestHeader::new(string).is_none());
}
// #[test]
// fn bad_request_line_2(){
//   let string = String::from("HTTP/1.1 GET A");
//   assert!(RequestHeader::new(string).is_none());
// }

#[test]
fn query_strings(){
  let string = String::from("GET /script/m.js?v=1 HTTP/1.1");
  let header = RequestHeader::new(string);
  assert!(header.is_some());
  let header = header.unwrap();
  let path = header.get_path();
  assert_eq!(path,"/script/m.js");
}

#[test]
fn uri_encoded() {
  let string = String::from("GET /script/%E4%B8%AD%E6%96%87.js HTTP/1.1");
  let header = RequestHeader::new(string);
  assert!(header.is_some());
  let header = header.unwrap();
  let path = header.get_path();
  assert_eq!(path,"/script/中文.js");
}