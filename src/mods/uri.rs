use super::utils::percent_decode;

pub struct URI<'a>{
  str:&'a str
}
impl<'a> URI<'a> {

  pub fn new(str:&'a str)->Self{
    URI{
      str
    }
  }
  pub fn decode(&self)->String{
    percent_decode(self.str)
  }


    
}