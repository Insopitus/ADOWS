use std::{io, net};

pub struct Server{
    listener:net::TcpListener,
    root_path:String,
}
impl Server {
    pub fn start(root_path:&str,port:u32)->Result<Self,io::Error>{
        let listener = net::TcpListener::bind(format!("http://localhost:{}",port))?;
        Ok(Server{
            listener,
            root_path:root_path.to_string()
        })
    }
}