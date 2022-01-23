use std::path::Path;
use std::{env, io};

use adows::{run};

fn main() {
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let path = env::args().skip(1).next().unwrap_or(current_dir);
    let port = 8080;
    run(port,path);
    
}

