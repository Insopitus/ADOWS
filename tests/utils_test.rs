use adows::mods::utils::percent_decode;



#[test]
fn decode_cjk_characters(){
    let string  = "%E5%85%A8%E5%9B%BD";
    assert_eq!(percent_decode(string),"全国".to_string())
}