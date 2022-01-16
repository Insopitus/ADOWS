use std::{
    io::{self, Read, Write, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use super::http_header::RequestHeader;

use super::folder_reader::FolderReader;

pub struct FileServer {
    reader: FolderReader,
}

impl FileServer {
    pub fn new(reader: FolderReader) -> FileServer {
        FileServer {
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
        let mut buf = Vec::with_capacity(1024);
        // stream.read(&mut buf)?;
        stream.read_to_end(&mut buf)?; //TODO don't need to read the full stream
        
        let http = RequestHeader::new(String::from_utf8_lossy(&buf).to_string()); // TODO utf8_lossy may cause the content-length mismatch
        dbg!(http.get_content_length());
        let code = 0;
        let path = http.get_path();
        let path = if path == "/" {
          "index.html" // redirect if path is empty
        }else{
          path
        };
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
