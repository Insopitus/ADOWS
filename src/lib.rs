pub mod mods;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path, sync::Arc,
};

use mods::{
    folder_reader::FolderReader,
    media_type::MediaType,
    request_header::RequestHeader,
    response_header::ResponseHeader,
    thread_pool::ThreadPool,
};




pub fn run(mut port: usize, path: String) {
    loop {
        let start = listen(port, path.clone());
        if let Err(e) = start {
            if e.kind() == io::ErrorKind::AddrInUse {
                port += 1;
                continue;
            } else {
                println!("Server failed to start");
                break;
            }
        } else {
            break;
        }
    }
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn listen(port: usize, path: String) -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port))?;
    let thread_pool = ThreadPool::new(5);
    println!("Server listening at http://localhost:{}", &port);

    // auto start the browser
    std::process::Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg(format!("http://localhost:{}", port))
        .spawn()
        .ok();


    let media_type_map = Arc::new(MediaType::new());
    let folder_reader = Arc::new(FolderReader::new(Path::new(&path)));
    for stream in listener.incoming() {
        let stream = stream?;
        let media_type_map = media_type_map.clone();
        let folder_reader = folder_reader.clone();
        thread_pool.execute(move|| {
            handle_connection(stream, folder_reader,media_type_map.clone()).ok();
        });
        // self.handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(stream: TcpStream, folder_reader: Arc<FolderReader>,media_type_map:Arc<MediaType>) -> Result<(), std::io::Error> {
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
        let mime_type;
        let suffix = path.split(".").last();
        if let Some(suffix) = suffix {
            mime_type = media_type_map
                .get_mime_type(suffix)
                .unwrap_or(&String::new())
                .to_owned();
        } else {
            mime_type = String::new();
        }
        let mut contents: Vec<u8>;
        match folder_reader.get_file_as_bytes(path) {
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
        send_response(stream, code, mime_type, &mut contents)?;
    } else {
        send_response(
            stream,
            400,
            String::new(),
            &mut "Bad Request".as_bytes().into(),
        )?;
    }

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = "<h1>Hi</h1>";
    Ok(())
}
fn send_response(
    mut stream: TcpStream,
    code: u32,
    media_type: String,
    contents: &mut Vec<u8>,
) -> Result<(), std::io::Error> {
    let mut response_header = ResponseHeader::new(code);
    if media_type != "" {
        response_header.insert_field("Content-Type".to_string(), media_type.to_string());
    }
    response_header.insert_field("Content-Length".to_string(), contents.len().to_string());
    response_header.insert_field("Server".to_string(), "A.D.O.W.S.".to_string());
    let response_header = response_header.to_string();
    let mut response = Vec::with_capacity(response_header.len() + contents.len());
    response.append(&mut response_header.as_bytes().into());
    response.append(contents);
    stream.write_all(&response)?;
    stream.flush()?;
    Ok(())
}

// pub struct FileServer {
//     reader: FolderReader,
//     media_type_map: MediaType,
//     thread_pool: ThreadPool,
// }

// impl FileServer {
//     pub fn new(path: &Path) -> FileServer {
//         let reader = FolderReader::new(path);
//         let media_type_map = MediaType::new();
//         let thread_pool = ThreadPool::new(5);
//         FileServer {
//             reader,
//             media_type_map,
//             thread_pool,
//         }
//     }
//     pub fn listen(&mut self, port: i32) -> Result<(), io::Error> {
//         let listener = TcpListener::bind(format!("127.0.0.1:{}", &port))?;
//         println!("Server listening at http://localhost:{}", &port);
//         std::process::Command::new("cmd.exe")
//             .arg("/C")
//             .arg("start")
//             .arg(format!("http://localhost:{}", port))
//             .spawn()
//             .ok();
//         for stream in listener.incoming() {
//             let stream = stream?;
//             // self.thread_pool.execute(|| {
//             //     self.handle_connection(stream);
//             // });
//             self.handle_connection(stream)?;
//         }
//         // }
//         Ok(())
//     }
//     fn handle_connection(&self, stream: TcpStream) -> Result<(), std::io::Error> {
//         let mut reader = BufReader::new(stream.try_clone()?);
//         let mut string = String::with_capacity(1024);

//         // reader.read_line(&mut string)?;
//         loop {
//             let line_size = reader.read_line(&mut string)?;
//             // println!("line size: {}",&line_size);
//             if line_size <= 2 {
//                 break; //break at the end of the header (an empty line with only b'\r\n')
//             }
//         }

//         let header = RequestHeader::new(string);
//         if let Some(header) = header {
//             let code;
//             let path = header.get_path();
//             let path = if path == "/" {
//                 "index.html" // redirect if path is empty
//             } else {
//                 path
//             };
//             let mime_type;
//             let suffix = path.split(".").last();
//             if let Some(suffix) = suffix {
//                 mime_type = self
//                     .media_type_map
//                     .get_mime_type(suffix)
//                     .unwrap_or(&String::new())
//                     .to_owned();
//             } else {
//                 mime_type = String::new();
//             }
//             let mut contents: Vec<u8>;
//             match self.reader.get_file_as_binary(path) {
//                 Ok(bytes) => {
//                     contents = bytes;
//                     code = 200;
//                 }
//                 Err(err) => match err.kind() {
//                     io::ErrorKind::NotFound => {
//                         contents = "Not Found".as_bytes().into();
//                         code = 404;
//                     }
//                     io::ErrorKind::PermissionDenied => {
//                         contents = "Forbiden".as_bytes().into();
//                         code = 403;
//                     }
//                     _ => {
//                         contents = "Forbiden".as_bytes().into();
//                         code = 403;
//                     }
//                 },
//             }
//             println!("Request: {} - {}", path, code);
//             FileServer::send_response(stream, code, mime_type, &mut contents)?;
//         } else {
//             FileServer::send_response(
//                 stream,
//                 400,
//                 String::new(),
//                 &mut "Bad Request".as_bytes().into(),
//             )?;
//         }

//         // let status_line = "HTTP/1.1 200 OK";
//         // let contents = "<h1>Hi</h1>";
//         Ok(())
//     }
//     pub fn send_response(
//         mut stream: TcpStream,
//         code: u32,
//         media_type: String,
//         contents: &mut Vec<u8>,
//     ) -> Result<(), std::io::Error> {
//         // TODO write a response header structure to orgnize the response
//         let mut response_header = ResponseHeader::new(code);
//         if media_type != "" {
//             response_header.insert_field("Content-Type".to_string(), media_type.to_string());
//         }
//         response_header.insert_field("Content-Length".to_string(), contents.len().to_string());
//         let response_header = response_header.to_string();
//         let mut response = Vec::with_capacity(response_header.len() + contents.len());
//         response.append(&mut response_header.as_bytes().into());
//         response.append(contents);
//         stream.write_all(&response)?;
//         stream.flush()?;
//         Ok(())
//     }
// }
