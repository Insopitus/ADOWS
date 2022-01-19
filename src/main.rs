use std::{env, io};
// use std::io::Write;
use std::path::Path;

mod mods;
use mods::{file_server::FileServer, folder_reader::FolderReader};

fn main() {
    // println!("Hello, world!");
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let path = env::args().skip(1).next().unwrap_or(current_dir);
    let path = Path::new(&path);
    let mut port = 8080;
    loop {
      let start = start_server(port,path);
      match start {
        Ok(_)=>{
          break;
        },
        Err(e)=>{
          match e.kind() {
              io::ErrorKind::AddrInUse=>{
                port+=1;
                continue;
              },
              _=>{
                println!("Server failed to start.");
                break
              }
          }
        }
      }
    }

    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
fn start_server(port: i32,path:&Path)->Result<(),io::Error> {
    let fr = FolderReader::new(path);
    let mut server = FileServer::new(fr);
    server.listen(port)
}
