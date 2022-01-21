use std::collections::HashMap;

pub struct HeaderFields{
    table:HashMap<String,String>
}
impl HeaderFields {
    pub fn new()->Self{
        HeaderFields{
            table:HashMap::new()
        }
    }
    pub fn from(s:String)->Self{
        let mut map = HashMap::new();
        let mut lines = s.split("\r\n");
        loop {
            let line = lines.next();
            match line {
                Some(pair) => match pair {
                    "" => break,
                    l => {
                        let mut split = l.split(":");
                        // dbg!(&split.collect::<Vec<_>>());

                        let k = split.next();
                        if k == None {
                            continue;
                        }
                        let k = k.unwrap().to_owned();
                        let v = split.next();
                        if v == None {
                            continue;
                        }
                        let v = v.unwrap().to_owned();

                        map.insert(k, v);
                    }
                },
                None => break,
            }
        }
        HeaderFields{
            table:map
        }
    }
    pub fn insert(&mut self,k:String,v:String){
        self.table.insert(k, v);
    }
    /// consumes the struct instance
    pub fn to_string(&self)->String{
        let mut result = String::new();
        for (key,value) in self.table.iter(){
            let line = format!("{}: {}\r\n",&key,&value);
            result.push_str(&line);
        }
        result
    }
    pub fn table(&self)->&HashMap<String,String>{
        &self.table
    }
}