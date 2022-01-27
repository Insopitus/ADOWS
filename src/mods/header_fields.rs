use std::collections::HashMap;

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
    pub fn table(&self) -> &HashMap<String, String> {
        &self.map
    }
}
impl From<String> for HeaderFields {
    fn from(s: String) -> Self {
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
                        value = v.to_owned();
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
