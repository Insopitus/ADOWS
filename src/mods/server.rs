use std::{
    io::{self, BufRead, BufReader, Write},
    net::{self, TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    mods::file_reader::{self, FileReader},
    THREAD_POOL_SIZE,
};

use super::{
    media_type::{self, MediaType},
    request_header::RequestHeader,
    response_header::ResponseHeader,
    thread_pool::ThreadPool,
};

pub struct Server {
    listener: net::TcpListener,
    root_path: String,
    port: u32,
    media_type_map: Arc<MediaType>,
}
impl Server {
    pub fn start(root_path: &str, port: u32) -> Result<Self, crate::error::Error> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = net::TcpListener::bind(addr)?;
        println!("Server listening at http://localhost:{}",port);
        let media_type_map: Arc<MediaType> = Arc::new(MediaType::new());
        let server = Server {
            listener,
            root_path: root_path.to_string(),
            port,
            media_type_map,
        };

        Server::init(&server)?;

        Ok(server)
    }
    fn init(server: &Server) -> Result<(), crate::error::Error> {
        // Server::open_browser(server.port);

        let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);
        for stream in server.listener.incoming() {
            let stream = stream?;
            let media_type_map = server.media_type_map.clone();
            let root_path = server.root_path.clone();
            thread_pool
                .execute(move || {
                    Server::handle_request(stream, media_type_map, root_path).unwrap();
                    // TODO may need handling
                }).unwrap();
        }
        Ok(())
    }
    /// auto start the browser
    fn open_browser(port: u32) {
        std::process::Command::new("cmd.exe")
            .arg("/C")
            .arg("start")
            .arg(format!("http://localhost:{}", port))
            .spawn()
            .ok();
    }
    fn handle_request(
        mut stream: TcpStream,
        media_type_map: Arc<MediaType>,
        root_path: String,
    ) -> Result<(), crate::error::Error> {
        let mut reader = BufReader::new(&mut stream);
        let mut string = String::with_capacity(1024);

        loop {
            let line_size = reader.read_line(&mut string).unwrap_or(0);
            // println!("line size: {}",&line_size);
            if line_size <= 2 {
                break; //break at the end of the header (an empty line with only b'\r\n')
            }
        }

        let header = RequestHeader::new(string);
        
        let (code, mime_type, content_length, path,entity_tag) = if let Some(header) = header {
            let default_etag = "unknown-input-etag".to_string();
            let request_etag = header.get_entity_tag().unwrap_or(default_etag);
            let code;
            let path = header.get_path().to_owned();
            let path = if path == "/" {
                "index.html".to_owned() // redirect if path is empty
            } else {
                path
            };
            print!("Request: {}", path);
            let mime_type;
            let suffix = path.split(".").last();
            if let Some(suffix) = suffix {
                mime_type = media_type_map.get_mime_type(suffix).unwrap_or("");
            } else {
                mime_type = "";
            }
            let mut content_length: usize = 0;
            match FileReader::new(&root_path, &path) {
                Ok(file_reader) => {
                    let file_etag = file_reader.get_entity_tag();
                    if request_etag == file_etag{
                        (304,mime_type.to_string(),0usize,None,request_etag.to_string())
                    }else{
                        match file_reader.get_size() {
                            Ok(length) => {
                                code = 200;
                                content_length = length.try_into().unwrap_or(0);
                            }
                            Err(err) => match err.kind() {
                                io::ErrorKind::NotFound => {
                                    code = 404;
                                }
                                io::ErrorKind::PermissionDenied => {
                                    code = 403;
                                }
                                _ => {
                                    code = 404;
                                }
                            },
                        }
                        println!(" - {}", code);

                        (code, mime_type.to_owned(), content_length, Some(path),file_reader.get_entity_tag())
                    }
                    
                }
                Err(_) => (404, "".to_owned(), 0, None, "".to_string()),
            }
        } else {
            (400, "".to_owned(), 0, None,"".to_string())
        };

        // send response headers
        let mut response_header = ResponseHeader::new(code);
        if mime_type != "" {
            response_header.insert_field("Content-Type".to_string(), mime_type.to_string());
        }
        response_header.insert_field("Content-Length".to_string(), content_length.to_string());
        response_header.insert_field("Server".to_string(), "A.D.O.W.S.".to_string());
        response_header.insert_field("Cache-Control".to_string(), "public".to_string());
        response_header.insert_field("ETag".to_string(), entity_tag);
        // response_header.insert_field("Connection".to_string(), "keep-alive".to_string());
        let response_header = response_header.to_string();
        let mut response = Vec::with_capacity(response_header.len() + content_length);
        response.append(&mut response_header.as_bytes().into());
        stream.write(&response)?;
        stream.flush()?;
        // send response body
        if let Some(path) = path {
            let mut file_reader = FileReader::new(&root_path, &path)?;

            for bytes in file_reader.read_chunked_as_bytes()? {
                stream.write(&bytes)?;
                stream.flush()?;
            }
        }
        Ok(())
    }
}
