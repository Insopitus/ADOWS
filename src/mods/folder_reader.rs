use std::{path::Path, fs::{self, read_dir}, io};

pub struct FolderReader{
    root_path:String,
}
impl FolderReader{
    pub fn new(path:&Path) -> FolderReader{
        let metadata = fs::metadata(path).unwrap();
        let root_path;
        if metadata.is_dir() {
            root_path = path.to_str().unwrap().to_string();
        }else{
            let str = path.to_str().unwrap();
            let mut a =str.split("\\").collect::<Vec<&str>>();
            a.pop();
            a.push("\\");
            root_path = a.join("\\");
        }
        FolderReader{
            root_path,
        }
    } 
    pub fn root_path(&self)->&str{
        &self.root_path
    }
    /// 
    fn get_full_path_from_relative(&self,dir:&str)->String{
        let mut file_path = self.root_path.clone();
        file_path.push_str(dir);
        file_path
    }
    pub fn get_file_as_string(&self,dir:&str)->Result<String,io::Error>{
        let file_path = self.get_full_path_from_relative(dir);
        fs::read_to_string(file_path)
    }
    /// recursively enumerate all the files in the path
    fn visit_dir(&self,path: &Path, info: &mut String) -> Result<(), std::io::Error> {
        for entry in read_dir(path)? {
            let entry = entry?;
            let dir = entry.path();
            if dir.is_dir() {
                self.visit_dir(&dir, info)?;
            } else if let Some(str) = dir.to_str() {
                info.push_str(str);
                info.push_str("\n")
            }
        }
        Ok(())
    }
}