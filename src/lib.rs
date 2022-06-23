pub mod cli;
mod concurrency;
mod error;
mod fs;
mod http;
mod server;
mod utils;
pub use server::Server;


const THREAD_POOL_SIZE: usize = 5;

pub fn run(mut port: u16, path: String) {
    loop {
        let start = Server::new(&path, port);
        match start {
            Err(e) => {
                if *e.kind() == error::ErrorKind::AddrInUse {
                    port += 1;
                    continue;
                } else {
                    println!("Server failed to start");
                    break;
                }
            }
            Ok(mut server) => {
                utils::open_browser(server.port);
                server.listen().unwrap(); // TODO error handling
                break;
            }
        };
    }
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap_or(0);
}
