use std::collections::HashMap;

pub struct MediaType{
  map:HashMap<String,String>
}
impl MediaType {
    pub fn new()->Self{
      let mut map = HashMap::new();
      // basic front end
      map.insert("html".to_string(), "text/html".to_string());
      map.insert("css".to_string(), "text/css".to_string());
      map.insert("js".to_string(), "text/javascript".to_string());
      map.insert("json".to_string(), "application/json".to_string());

      // images
      map.insert("jpg".to_string(), "image/jpeg".to_string());
      map.insert("jpeg".to_string(), "image/jpeg".to_string());
      map.insert("png".to_string(), "image/png".to_string());
      map.insert("svg".to_string(), "image/svg+xml".to_string());
      map.insert("gif".to_string(), "image/gif".to_string());
      map.insert("webp".to_string(), "image/webp".to_string());

      // videos
      map.insert("mp4".to_string(), "video/mp4".to_string());
      map.insert("avi".to_string(), "video/avi".to_string());





      MediaType{
        map
      }
    }

    pub fn get_mime_type(&self,suffix:&str)->Option<&String>{
      self.map.get(suffix)
    }
}