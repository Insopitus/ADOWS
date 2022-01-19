use std::env;
// use std::io::Write;
use std::path::Path;

mod mods;
use mods::{file_server::FileServer, folder_reader::FolderReader};

fn main() {
    // println!("Hello, world!");
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let path = env::args().skip(1).next().unwrap_or(current_dir);
    let path = Path::new(&path);
    // let mut info = String::new();
    // if path.is_dir() {
    //     visit_dir(path, &mut info).unwrap();
    // } else if path.is_file() {
    //     // info.push_str("is file")
    //     info.push_str(path.to_str().unwrap());
    // }
    // // // if write to a file
    // // let mut log_file = File::create("log.txt").unwrap();
    // // log_file.write_all(info.as_bytes()).unwrap_or_else(|_| {});

    // // else print in the console
    // println!("{}",info);
    let fr = FolderReader::new(path);
    let mut server = FileServer::new(fr);
    server
        .listen(8080)
        .expect("Error occured. Server is closing.");
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
