use std::{
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    time::SystemTime,
};

const CHUNK_SIZE: usize = 1024 * 64; // 64kb
#[derive(Debug)]
pub struct FileReader {
    path: PathBuf,
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(root_path: &str, path: &str) -> Result<Self, io::Error> {
        let root_path = Path::new(root_path);
        let mut sub_path = Path::new(path);
        sub_path = sub_path.strip_prefix("/").unwrap_or(sub_path);
        sub_path = sub_path.strip_prefix("./").unwrap_or(sub_path);

        let path = root_path.join(sub_path);

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        Ok(FileReader { path, reader })
    }
    pub fn get_size(&self) -> Result<u64, io::Error> {
        Ok(fs::metadata(&self.path)?.len())
    }
    pub fn _read_as_string(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.path)
    }
    pub fn _read_as_bytes(&self) -> Result<Vec<u8>, io::Error> {
        fs::read(&self.path)
    }
    pub fn read_chunked_as_bytes(&mut self) -> Result<FileChunksReader, io::Error> {
        let metadata = fs::metadata(&self.path)?;
        let length = metadata.len();
        let chunks = FileChunksReader {
            reader: &mut self.reader,
            bytes_remaining: length,
        };
        Ok(chunks)
    }
    fn get_last_modified_time(&self) -> Result<SystemTime, io::Error> {
        let metadata = fs::metadata(&self.path)?;
        metadata.modified()
    }
    /// for HTML ETag header. based on the last modified time for the moment
    pub fn get_entity_tag(&self) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        match self.get_last_modified_time() {
            Ok(time) => {
                time.hash(&mut hasher);
                format!("{:x}", hasher.finish())
            }
            Err(_) => "unknown-time".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FileChunksReader<'a> {
    reader: &'a mut BufReader<File>,
    bytes_remaining: u64,
}
impl<'a> Iterator for FileChunksReader<'a> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut content: Vec<u8>;
        let chunk_size = CHUNK_SIZE.try_into().unwrap_or(1024 * 64);
        // be careful of integer overflow
        if self.bytes_remaining > chunk_size {
            content = vec![0u8; CHUNK_SIZE];
            self.bytes_remaining -= CHUNK_SIZE.try_into().unwrap_or(1024 * 64);
        } else if self.bytes_remaining > 0 {
            content = vec![0u8; self.bytes_remaining.try_into().unwrap_or_default()];
            self.bytes_remaining = 0;
        } else {
            return None;
        }

        match self.reader.read(&mut content) {
            Ok(_) => Some(content),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileReader;

    #[test]
    fn file_dont_exist() {
        let reader = FileReader::new("", "tests/assets/404.txt");
        assert!(reader.is_err());
    }
    #[test]
    fn basic() {
        let reader = FileReader::new("", "tests/assets/file-reader.txt");
        assert!(reader.is_ok());
        let reader = reader.unwrap();
        let result = reader._read_as_string();
        assert!(result.is_ok());
        assert_eq!(reader._read_as_string().unwrap(), "Hello, Reader");
        assert_eq!(reader._read_as_bytes().unwrap(), b"Hello, Reader");
    }
    #[test]
    fn chunked() {
        let reader = FileReader::new("", "tests/assets/file-reader.txt");
        assert!(reader.is_ok());
        let mut reader = reader.unwrap();
        let mut result: Vec<u8> = Vec::new();
        for chunk in reader.read_chunked_as_bytes().unwrap() {
            for data in chunk {
                result.push(data);
            }
        }
        assert_eq!(result, b"Hello, Reader");
    }
}
