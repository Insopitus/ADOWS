pub struct HTTPHeader {
  content:String,
  path: String,
  method:String,
  version:String,
}
impl HTTPHeader {
  pub fn new(string:String)->Self{
    let content = string.to_owned();
    println!("{}",&content);
    let line_one = string.split("\r\n").nth(0);
    let line_one = line_one.expect("Invalid HTTP Content.");
    let mut line_one_iter = line_one.split(" ");
    let method = line_one_iter.next().expect("Invalid HTTP Content.").to_string();
    let path = line_one_iter.next().expect("Invalid HTTP Content.").to_string();
    let version = line_one_iter.next().expect("Invalid HTTP Content.").to_string();
    HTTPHeader {
      content,
      path,
      method,
      version,
    }
  }
  pub fn get_method(&self)->&String{
    &self.method
  }
  pub fn get_path(&self)->&String{
    &self.path
  }
  pub fn get_version(&self)->&String{
    &self.version
  }
  pub fn is_get(&self)->bool{
    self.method == "GET"
  }
}