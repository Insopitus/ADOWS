use std::{net::{TcpListener, TcpStream, }, fmt::format, thread, time::Duration, io::{Read, Write}, fs};

use super::folder_reader::FolderReader;

pub struct FileServer{
    listener:TcpListener,
    reader:FolderReader,
}

impl FileServer{
    pub fn start(port:i32,reader:FolderReader)->FileServer{
        let listener = TcpListener::bind(format!("127.0.0.1:{}",&port)).unwrap();

        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();
            println!("request incoming");
            handle_connection(stream);
        }

        FileServer{
            listener,
            reader,
        }
    }
    pub fn get_file_from_request(&self){

    }

}

fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 1024];
    let mut string = String::new();
    
    
    stream.read_to_string(&mut string).unwrap();

    println!("{}",string);
    // let get = b"GET / HTTP/1.1\r\n";
    // let sleep = b"GET /sleep HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };

    // let contents = fs::read_to_string(filename).unwrap();

    // let response = format!("{}{}", status_line, contents);

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}