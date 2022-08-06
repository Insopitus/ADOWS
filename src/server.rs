use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::{self, TcpStream},
    sync::Arc, path::PathBuf,
};

use crate::{THREAD_POOL_SIZE, cli::Config};

use crate::{
    concurrency::ThreadPool,
    error,
    fs::FileReader,
    http::{MediaType, RequestHeader, ResponseHeader},
};

pub struct Server {
    listener: net::TcpListener,
    root_path: PathBuf,
    pub port: u16,
    cross_origin: bool,
    media_type_map: Arc<MediaType>,
}
impl Server {
    /// create a new server instance
    pub fn new(config: &Config) -> Result<Self, error::Error> {
        let addr = format!("127.0.0.1:{}", config.port);
        let listener = net::TcpListener::bind(addr)?;
        println!("Server listening at http://localhost:{}", config.port);
        let media_type_map: Arc<MediaType> = Arc::new(MediaType::new());
        let server = Server {
            listener,
            root_path: config.dir.clone(),
            port:config.port,
            media_type_map,
            cross_origin:config.cross_origin
        };

        Ok(server)
    }
    /// listening connections
    pub fn listen(&mut self) -> Result<(), error::Error> {
        let thread_pool = ThreadPool::new(THREAD_POOL_SIZE);
        for stream in self.listener.incoming() {
            let stream = stream?;
            let media_type_map = self.media_type_map.clone();
            let root_path = self.root_path.clone();
            let cross_origin = self.cross_origin;
            thread_pool
                .execute(move || {
                    match Server::handle_request(stream, media_type_map, root_path, cross_origin) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                    // TODO may need handling
                })
                .unwrap();
        }
        Ok(())
    }

    fn handle_request(
        stream: TcpStream,
        media_type_map: Arc<MediaType>,
        root_path: PathBuf,
        cross_origin: bool
    ) -> Result<(), error::Error> {
        let request_header = Server::parse_request(&stream);
        let mut file_reader = None; // TODO use the same file reader instance
        let mut response_header = ResponseHeader::new(400);
        let mut content_length = 0usize;
        let mut path: &str;
        if let Some(request_header) = request_header {
            let default_etag = "unknown-input-etag".to_string();
            let request_etag = request_header.get_entity_tag().unwrap_or(default_etag);
            path = request_header.get_path();
            path = if path == "/" {
                "index.html" // redirect if path is empty
            } else {
                path
            };
            print!("Request: {}", path);

            // mime type
            if let Some((_, ext)) = path.rsplit_once('.') {
                let mime_type = media_type_map.get_mime_type(ext).unwrap_or("");
                if !mime_type.is_empty() {
                    response_header.insert_field("Content-Type".to_string(), mime_type.to_string());
                }
            }
            match FileReader::new(&root_path, path) {
                Ok(reader) => {
                    let file_etag = reader.get_entity_tag();
                    let code: u32;

                    if request_etag == file_etag {
                        code = 304;
                    } else {
                        match reader.get_size() {
                            Ok(length) => {
                                code = 200;
                                content_length = length.try_into().unwrap_or_default();
                                response_header
                                    .insert_field("Content-Length".to_string(), length.to_string());
                            }
                            Err(err) => {
                                dbg!(&err);
                                match err.kind() {
                                    io::ErrorKind::NotFound => {
                                        code = 404;
                                    }
                                    io::ErrorKind::PermissionDenied => {
                                        code = 403;
                                    }
                                    _ => {
                                        code = 404;
                                    }
                                }
                            }
                        }

                        response_header.insert_field("ETag".to_string(), reader.get_entity_tag());
                    }
                    println!(" - {}", code);
                    response_header.code = code;
                    file_reader = Some(reader);
                }
                Err(_) => {
                    response_header.code = 404;
                }
            };
        } else {
            response_header.code = 400;
        }

        if cross_origin {
            response_header.insert_field("Access-Control-Allow-Origin".to_string(), "*".to_string());
        }

        // send response headers
        // dbg!(&response_header);

        let response_header_string = response_header.to_string();
        let mut response = Vec::with_capacity(response_header_string.len() + content_length);
        response.append(&mut response_header_string.as_bytes().into());
        let mut writer = BufWriter::new(&stream);
        writer.write_all(&response)?;
        writer.flush()?;
        // send response body
        if response_header.code == 200 {
            if let Some(mut reader) = file_reader {
                for bytes in reader.read_chunked_as_bytes()? {
                    writer.write_all(&bytes)?;
                    writer.flush()?;
                }
            }
        }

        Ok(())
    }

    /// parse the request head and return a RequestHeader struct
    fn parse_request(mut stream: &TcpStream) -> Option<RequestHeader> {
        let mut reader = BufReader::new(&mut stream);
        let mut string = String::with_capacity(1024);

        loop {
            let line_size = reader.read_line(&mut string).unwrap_or(0);
            // println!("line size: {}",&line_size);
            if line_size <= 2 {
                break; //break at the end of the header (an empty line with only b'\r\n')
            }
        }
        RequestHeader::new(string)
    }

    // fn create_response_header(code:u32,media_type_map: Arc<MediaType>)->ResponseHeader{
    //     let mime_type:&str;
    //     let content_length:usize;
    //     let entity_tag:String;
    //     let path:&str;
    //     if header.is_none() {
    //         code = 400;
    //         mime_type = "";
    //         content_length = 0;
    //         entity_tag = "".to_string();
    //         path = "";
    //     }else{
    //         let header = header.unwrap();
    //         let default_etag = "unknown-input-etag".to_string();
    //         let request_etag = header.get_entity_tag().unwrap_or(default_etag);
    //         let path_string = header.get_path().to_owned();
    //         path = if path_string == "/" {
    //             "index.html" // redirect if path is empty
    //         } else {
    //             &path_string
    //         };
    //         print!("Request: {}", path);
    //         let suffix = path.split(".").last();
    //         if let Some(suffix) = suffix {
    //             mime_type = media_type_map.get_mime_type(suffix).unwrap_or("");
    //         } else {
    //             mime_type = "";
    //         }
    //         match FileReader::new(root_path, &path) {
    //             Ok(file_reader) => {
    //                 let file_etag = file_reader.get_entity_tag();
    //                 if request_etag == file_etag{
    //                     code = 304;
    //                     content_length = 0;
    //                     entity_tag = "".to_string();
    //                 }else{
    //                     match file_reader.get_size() {
    //                         Ok(length) => {
    //                             code = 200;
    //                             content_length = length.try_into().unwrap_or(0);
    //                         }
    //                         Err(err) => {
    //                             match err.kind() {
    //                                 io::ErrorKind::NotFound => {
    //                                     code = 404;
    //                                 }
    //                                 io::ErrorKind::PermissionDenied => {
    //                                     code = 403;
    //                                 }
    //                                 _ => {
    //                                     code = 404;
    //                                 }
    //                             }
    //                             content_length = 0;
    //                         }
    //                     }
    //                     println!(" - {}", code);
    //                     entity_tag = file_reader.get_entity_tag();
    //                 }

    //             }
    //             Err(_) =>{
    //                 code = 404;
    //                 content_length = 0;
    //                 entity_tag = "".to_string();
    //             }
    //         };
    //     }
    //     let mut response_header = ResponseHeader::new(code);

    //     // don't send header fields if use 304 cache
    //     if code != 304 {
    //         if mime_type != "" {
    //             response_header.insert_field("Content-Type".to_string(), mime_type.to_string());
    //         }
    //         if entity_tag != "" {
    //             response_header.insert_field("ETag".to_string(), entity_tag.to_string());
    //         }
    //         response_header.insert_field("Content-Length".to_string(), content_length.to_string());
    //         response_header.insert_field("Server".to_string(), "A.D.O.W.S.".to_string());
    //         response_header.insert_field("Cache-Control".to_string(), "public".to_string());
    //         // response_header.insert_field("Connection".to_string(), "keep-alive".to_string());
    //     }
    //     (response_header,path,content_length)

    // }
}
