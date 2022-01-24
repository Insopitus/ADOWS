use std::env;

fn main() {
    
    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    let path = env::args().skip(1).next().unwrap_or(current_dir);
    let port = 8080;
    adows::run(port,path);
    
}

