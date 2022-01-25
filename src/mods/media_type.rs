use std::collections::HashMap;

pub struct MediaType{
  map:HashMap<&'static str,&'static str>
}
impl MediaType {
    pub fn new()->Self{
      let mut map = HashMap::new();
      // basic front end
      map.insert("html", "text/html");
      map.insert("css", "text/css");
      map.insert("js", "text/javascript");
      map.insert("json", "application/json");

      // images
      map.insert("jpg", "image/jpeg");
      map.insert("jpeg", "image/jpeg");
      map.insert("png", "image/png");
      map.insert("svg", "image/svg+xml");
      map.insert("gif", "image/gif");
      map.insert("webp", "image/webp");

      // videos
      map.insert("mp4", "video/mp4");
      map.insert("avi", "video/avi");





      MediaType{
        map
      }
    }

    pub fn get_mime_type(&self,suffix:&str)->Option<&str>{
      self.map.get(suffix).map(|s|*s)
    }
}