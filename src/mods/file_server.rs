use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use super::http_header::RequestHeader;

use super::folder_reader::FolderReader;

pub struct FileServer {
    reader: FolderReader,
}

impl FileServer {
    pub fn new(reader: FolderReader) -> FileServer {
        FileServer { reader }
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
            // println!("Request incoming.");

            let stream = stream?;
            self.handle_connection(stream)?;
        }
        // }
        Ok(())
    }
    fn handle_connection(&self, stream: TcpStream) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut string = String::with_capacity(1024);

        // reader.read_line(&mut string)?;
        loop {
            let line_size = reader.read_line(&mut string)?;
            // println!("line size: {}",&line_size);
            if line_size <= 2 {
                break; //break at the end of the header (an empty line with only b'\r\n')
            }
        }

        let header = RequestHeader::new(string);
        if let Some(header) = header {
            let code;
            let path = header.get_path();
            let path = if path == "/" {
                "index.html" // redirect if path is empty
            } else {
                path
            };
            let mut contents: Vec<u8>;
            match self.reader.get_file_as_binary(path) {
                Ok(bytes) => {
                    contents = bytes;
                    code = 200;
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => {
                        contents = "Not Found".as_bytes().into();
                        code = 404;
                    }
                    io::ErrorKind::PermissionDenied => {
                        contents = "Forbiden".as_bytes().into();
                        code = 403;
                    }
                    _ => {
                        contents = "Forbiden".as_bytes().into();
                        code = 403;
                    }
                },
            }
            println!("Request: {} - {}", path, code);
            FileServer::send_response(stream, code, &mut contents)?;
        } else {
            FileServer::send_response(stream, 400, &mut "Bad Request".as_bytes().into())?;
        }

        // let status_line = "HTTP/1.1 200 OK";
        // let contents = "<h1>Hi</h1>";
        Ok(())
    }
    pub fn send_response(
        mut stream: TcpStream,
        code: u32,
        contents: &mut Vec<u8>,
    ) -> Result<(), std::io::Error> {
        // TODO write a response header structure to orgnize the response
        let status_line = if code == 200 {
            "HTTP/1.1 200 OK"
        } else if code == 404 {
            "HTTP/1.1 404 NOT FOUND"
        } else {
            "HTTP/1.1 404 NOT FOUND"
        };
        let response_header = format!(
            "{}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            contents.len(),
        );
        let mut response = Vec::with_capacity(response_header.len() + contents.len());
        response.append(&mut response_header.as_bytes().into());
        response.append(contents);
        stream.write_all(&response)?;
        stream.flush()?;
        // println!("Response sent: \r\n{}\r\n", response);
        Ok(())
    }
}
