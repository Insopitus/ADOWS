pub mod cli;
mod concurrency;
mod error;
mod fs;
mod http;
mod server;
mod utils;
use cli::Config;
pub use server::Server;


const THREAD_POOL_SIZE: usize = 5;

pub fn run(mut config:Config) {
    loop {
        let start = Server::new(&config);
        match start {
            Err(e) => {
                if *e.kind() == error::ErrorKind::AddrInUse {
                    config.port += 1;
                    continue;
                } else {
                    println!("Server failed to start");
                    break;
                }
            }
            Ok(mut server) => {

                if config.open_browser { utils::open_browser(server.port); }
                server.listen().unwrap(); // TODO error handling
                break;
            }
        };
    }
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap_or(0);
}
