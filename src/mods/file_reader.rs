use std::{path::{Path, PathBuf}, io::{BufReader, self, Read}, fs::{File, self}};

const CHUNK_SIZE: usize = 1024*64; // 64kb
#[derive(Debug)]
pub struct FileReader{
    path:PathBuf,
    reader:BufReader<File>
}

impl FileReader {
    pub fn new(root_path:&str,path:&str)->Result<Self,io::Error>{
        let path = Path::new(root_path).join(path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        Ok(FileReader{
            path,
            reader
        })
    }
    pub fn get_file_size(&self) -> Result<u64, io::Error> {
        Ok(fs::metadata(&self.path)?.len())
    }
    pub fn read_as_string(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.path)
    }
    pub fn read_as_bytes(&self) -> Result<Vec<u8>, io::Error> {
        fs::read(&self.path)
    }
    // pub fn read_chunked_as_bytes(&mut self) -> Result<FileChunksReader, io::Error> {
    //     let metadata = fs::metadata(&self.path)?;
    //     let length = metadata.len();
    //     let chunks = FileChunksReader {
    //         reader: &mut self.reader,
    //         bytes_remaining: length.try_into().unwrap_or(0),
    //     };
    //     Ok(chunks)
    // }

}

// #[derive(Debug)]
// pub struct FileChunksReader<'a> {
//     reader: &'a mut BufReader<File>,
//     bytes_remaining: i64,
// }
// impl<'a> Iterator for FileChunksReader<'a> {
//     type Item = [u8; CHUNK_SIZE];
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.bytes_remaining > 0 {
//             match self.reader.read(&mut self.content) {
//                 Ok(_) => {
//                     self.bytes_remaining -= CHUNK_SIZE.try_into().unwrap_or(1024*64);
//                     Some(self.content)
//                 }
//                 Err(e) => match e.kind() {
//                     io::ErrorKind::UnexpectedEof => None,
//                     _ => None,
//                 },
//             }
//         } else {
//             None
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::FileReader;

    #[test]
    fn file_dont_exist(){
        let reader = FileReader::new("","tests/assets/404.txt");
        assert!(reader.is_err());
    }
    #[test]
    fn basic(){
        let reader = FileReader::new("","tests/assets/file-reader.txt");
        assert!(reader.is_ok());
        let reader = reader.unwrap();
        let result = reader.read_as_string();
        assert!(result.is_ok());
        assert_eq!(reader.read_as_string().unwrap(),"Hello, Reader\r\n");
    }
    // #[test]
    // fn chunked(){
    //     let reader = FileReader::new("","tests/assets/file-reader.txt");
    //     assert!(reader.is_ok());
    //     let mut reader = reader.unwrap();
    //     let mut result:Vec<u8> = Vec::new();
    //     for chunk in reader.read_chunked_as_bytes().unwrap() {
    //         for data in chunk {
    //             result.push(data);
    //         }
    //     }
    //     assert_eq!(result,b"Hello, Reader\r\n");
    // }
}