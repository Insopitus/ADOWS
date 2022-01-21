use adows::mods::utils::percent_decode;



#[test]
fn decode_cjk_characters(){
    let string  = "%E5%85%A8%E5%9B%BD";
    assert_eq!(percent_decode(string),"全国".to_string())
}
#[test]
fn decode_reserved_characters(){
    let string = "%7B%22a%22:1,%22b%22:%22bar%22%7D";
    assert_eq!(percent_decode(string),"{\"a\":1,\"b\":\"bar\"}");
}
// TODO
// #[test]
// fn decode_none_percent_encoded(){
//     let string = "%7s";
//     assert_eq!(percent_decode(string),"%7s");
// }
