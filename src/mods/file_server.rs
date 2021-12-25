use std::{
    fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::mods::http_header::HTTPHeader;

use super::folder_reader::FolderReader;

pub struct FileServer {
    listener: Option<TcpListener>,
    reader: FolderReader,
}

impl FileServer {
    pub fn new(reader: FolderReader) -> FileServer {
        FileServer {
            listener: None,
            reader,
        }
    }
    pub fn listen(&mut self, port: i32) -> Result<(), io::Error> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", &port))?;
        println!("Server listening at http://localhost:{}", &port);
        // self.listener = Some(listener);
        // if let Some(listener) = listener {
        // loop {
        //     let (stream, addr) = listener.accept()?;
        //     FileServer::handle_connection(stream);
        // }
        for stream in listener.incoming() {
            println!("Request incoming.");

            let stream = stream?;
            self.handle_connection(stream)?;
        }
        // }
        Ok(())
    }
    pub fn get_file_from_request(&self) {}
    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), std::io::Error> {
        let mut buf = [0u8; 1024]; //TODO accept arbitrary length
        stream.read(&mut buf)?;
        let http = HTTPHeader::new(String::from_utf8_lossy(&buf).to_string());
        let code = 0;
        let path = http.get_path();
        let path = if path == "/" {
          "index.html"
        }else{
          path
        };
        println!("{}",path);
        let contents = self.reader.get_file_as_string(path)?;
        // let status_line = "HTTP/1.1 200 OK";
        // let contents = "<h1>Hi</h1>";
        FileServer::send_response(stream, code, contents)?;
        Ok(())
    }
    pub fn send_response(
        mut stream: TcpStream,
        code: u32,
        contents: String,
    ) -> Result<(), std::io::Error> {
        let status_line = if code == 200 {
            "HTTP/1.1 200 OK"
        } else if code == 404 {
            "HTTP/1.1 404 NOT FOUND"
        } else {
            "HTTP/1.1 404 NOT FOUND"
        };
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes())?;
        stream.flush()?;
        println!("Response sent: \r\n{}\r\n", response);
        Ok(())
    }
}
