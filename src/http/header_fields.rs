use std::collections::HashMap;

/// header fileds for request/response headers
#[derive(Debug)]
pub struct HeaderFields {
    map: HashMap<String, String>,
}
impl HeaderFields {
    pub fn new() -> Self {
        HeaderFields {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: String) {
        self.map.insert(k, v);
    }
    /// consumes the struct instance
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in self.map.iter() {
            let line = format!("{}: {}\r\n", &key, &value);
            result.push_str(&line);
        }
        result
    }
    /// return the hashmap of field pairs
    pub fn table(&self) -> &HashMap<String, String> {
        &self.map
    }

    pub fn get(&self,key:&str)->Option<&String>{
        self.map.get(key)
    }
}
impl From<&str> for HeaderFields {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();
        for line in s.lines() {
            match line {
                "" => break,
                l => {
                    let mut split = l.split(":");
                    // dbg!(&split.collect::<Vec<_>>());

                    let key;
                    let value;
                    let k = split.next();
                    if let Some(k) = k {
                        key = k.to_owned();
                    } else {
                        continue;
                    }
                    let v = split.next();
                    if let Some(v) = v {
                        value = v.trim().to_owned();
                    } else {
                        continue;
                    }

                    map.insert(key, value);
                }
            }
        }

        HeaderFields { map }
    }
}

#[cfg(test)]
mod test {
    use super::HeaderFields;

    #[test]
    fn from_string() {
        let line = "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
        let field = HeaderFields::from(line);
        assert_eq!(field.table().get("Accept").unwrap(),"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9");
    }
    #[test]
    fn from_multi_line_string() {
        let lines = "Accept-Encoding: gzip, deflate, br\r\nAccept-Language: zh-CN,zh;q=0.9,en;q=0.8\r\nCache-Control: max-age=0\r\nConnection: keep-alive";
        let field = HeaderFields::from(lines);
        assert_eq!(field.table().get("Accept-Encoding").unwrap(),"gzip, deflate, br");
        assert_eq!(field.table().get("Accept-Language").unwrap(),"zh-CN,zh;q=0.9,en;q=0.8");
        assert_eq!(field.table().get("Cache-Control").unwrap(),"max-age=0");
        assert_eq!(field.table().get("Connection").unwrap(),"keep-alive");
    }

    #[test]
    fn from_broken_string(){
        let line = "Sec-Fetch-Dest:\r\nSec-Fetch-Site";
        let field = HeaderFields::from(line);
        let map = field.table();
        assert_eq!(map.get("Sec-Fetch-Dest").unwrap(),"");
        assert_eq!(map.get("Sec-Fetch-Site"),None);
    }

    #[test]
    fn to_string(){
        let mut field = HeaderFields::new();
        field.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        assert_eq!(field.to_string(),"Upgrade-Insecure-Requests: 1\r\n")
    }
}
