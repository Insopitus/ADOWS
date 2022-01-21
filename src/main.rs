use std::path::Path;
use std::{env, io};

use adows::FileServer;

fn main() {
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let path = env::args().skip(1).next().unwrap_or(current_dir);
    let path = Path::new(&path);
    let mut port = 8080;
    let mut server = FileServer::new(path);
    loop {
        let start = server.listen(port);
        if let Err(e)=start {
          if e.kind() == io::ErrorKind::AddrInUse {
              port += 1;
              continue;
          } else {
              println!("Server failed to start.");
              break;
          }
        } else {
            break;
        }
    }
    println!("Press Enter to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

