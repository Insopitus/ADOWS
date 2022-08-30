/// percent decoding for URIs
///
/// returns the original characters when meeting invalid percent encoding e.g. "%7s"->"%7s"
pub fn percent_decode(string: &str) -> String {
    let mut bytes = string.as_bytes().iter();
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());

    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let first;
                let second;
                match bytes.next() {
                    Some(b) => {
                        first = *b;
                        match bytes.next() {
                            Some(b) => {
                                second = *b;
                                let first_int = ascii_hex_char_byte_to_number(first);

                                let second_int = ascii_hex_char_byte_to_number(second);
                                if first_int.is_none() || second_int.is_none() {
                                    result.push(b'%');
                                    result.push(first);
                                    result.push(second);
                                    continue;
                                }
                                let first_int = first_int.unwrap();
                                let second_int = second_int.unwrap();
                                let byte = first_int << 4 | second_int;
                                result.push(byte);
                            }
                            None => {
                                result.push(b'%');
                                result.push(first);
                            }
                        }
                    }
                    None => {
                        result.push(b'%');
                    }
                }
            }
            other => {
                result.push(*other);
            }
        }
    }

    String::from_utf8_lossy(&result).to_string()
}

/// convert a ascii character that represents a hex value
///  to its numeric value, e.g. b'A' -> 10
///
/// returns Options::None if it's not a valid Hex character e.g. b'U'
fn ascii_hex_char_byte_to_number(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// auto start the browser (windows and linux)
pub fn open_browser(port: u16) {
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(format!("http://localhost:{}", port))
        .spawn()
        .ok();// if it fails, it fails.
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg(format!("http://localhost:{}", port))
        .spawn()
        .ok(); // if it fails, it fails.
}

#[cfg(test)]
mod test_decode {
    use super::percent_decode;

    #[test]
    fn decode_cjk_characters() {
        let string = "%E5%85%A8%E5%9B%BD";
        assert_eq!(percent_decode(string), "全国".to_string())
    }
    #[test]
    fn decode_reserved_characters() {
        let string = "%7B%22a%22:1,%22b%22:%22bar%22%7D";
        assert_eq!(percent_decode(string), "{\"a\":1,\"b\":\"bar\"}");
    }
    #[test]
    fn decode_none_percent_encoded() {
        let string = "%7s";
        assert_eq!(percent_decode(string), "%7s");
    }
}
