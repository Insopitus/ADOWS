/// percent decoding for URIs
pub fn percent_decode(string: &str) -> String {
    let mut bytes = string.as_bytes().into_iter();
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    loop {
        match bytes.next() {
            Some(b) => match b {
                b'%' => {
                    let first;
                    let second;
                    match bytes.next() {
                        Some(b) => {
                            first = *b;
                            match bytes.next() {
                                Some(b) => {
                                    second = *b;
                                    let first_int = ascii_to_hex(first);

                                    let second_int = ascii_to_hex(second);
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
            },
            None => {
                break;
            }
        }
    }

    String::from_utf8_lossy(&result).to_string()
}

/// convert a utf-8 byte to its hex value, e.g. b'A' -> 10
pub fn ascii_to_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
