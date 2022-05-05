use std::{
    io::{self, BufRead, BufReader, Write},
    net::{self, TcpStream},
    sync::Arc,
};

use crate::{
    mods::file_reader::{FileReader},
    THREAD_POOL_SIZE,
};

use super::{
    media_type::{MediaType},
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
                    match Server::handle_request(stream, media_type_map, root_path){
                        Ok(_)=>{},
                        Err(e)=>{
                            println!("{}",e);
                        }
                    }
                    // TODO may need handling
                }).unwrap();
        }
        Ok(())
    }
    
    fn handle_request(
        mut stream: TcpStream,
        media_type_map: Arc<MediaType>,
        root_path: String,
    ) -> Result<(), crate::error::Error> {
        let request_header = Server::parse_request(&mut stream);
        let mut file_reader = None; // TODO use the same file reader instance
        let mut response_header = ResponseHeader::new(400);
        let mut content_length = 0usize;
        let mut path = String::new();
        if let Some(request_header) = request_header {
            let default_etag = "unknown-input-etag".to_string();
            let request_etag = request_header.get_entity_tag().unwrap_or(default_etag);
            path = request_header.get_path().to_string();
            path = if path == "/" {
                "index.html".to_string() // redirect if path is empty
            } else {
                path
            };
            print!("Request: {}", path);

            // mime type
            let suffix = path.split(".").last();
            if let Some(suffix) = suffix {
                let mime_type = media_type_map.get_mime_type(suffix).unwrap_or("");
                if mime_type != "" {
                    response_header.insert_field("Content-Type".to_string(), mime_type.to_string());
                }
            } 
            match FileReader::new(&root_path, &path) {
                Ok(reader) => {
                    let file_etag = reader.get_entity_tag();
                    let code:u32;
                    
                    if request_etag == file_etag{
                        code = 304;
                    }else{
                        match reader.get_size() {
                            Ok(length) => {
                                code = 200;
                                content_length = length.try_into().unwrap_or_default();
                                response_header.insert_field("Content-Length".to_string(), length.to_string());
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
                        
                        response_header.insert_field("ETag".to_string(), reader.get_entity_tag().to_string());
                    }
                    println!(" - {}", code);
                    response_header.set_code(code);
                    file_reader = Some(reader);
                }
                Err(_) =>{
                    response_header.set_code(404);
                } 
            };

        }else{
            response_header.set_code(400);
        }

        
        

        
        // send response headers
        // dbg!(&response_header);
        
        let response_header = response_header.to_string();
        let mut response = Vec::with_capacity(response_header.len() + content_length);
        response.append(&mut response_header.as_bytes().into());
        stream.write(&response)?;
        stream.flush()?;
        // send response body
        if let Some(mut reader) = file_reader {            

            for bytes in reader.read_chunked_as_bytes()? {
                stream.write(&bytes)?;
                stream.flush()?;
            }
        }
        Ok(())
    }

    /// parse the request head and return a RequestHeader struct
    fn parse_request(mut stream:&TcpStream)->Option<RequestHeader>{
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
