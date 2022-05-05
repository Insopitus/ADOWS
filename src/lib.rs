pub mod error;
pub mod mods;




use crate::mods::server::Server;
const THREAD_POOL_SIZE: usize = 5;
pub fn run(mut port: u32, path: String) {
    loop {
        let start = Server::start(&path, port);
        if let Err(e) = start {
            if *e.kind() == crate::error::ErrorKind::AddrInUse {
                port += 1;
                continue;
            } else {
                println!("Server failed to start");
                break;
            }
        } else {
            // println!("Open browser");
            // mods::utils::open_browser(port);
            break;
        }
    }
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap_or(0);
}


