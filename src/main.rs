use std::env;

use adows::cli::Config;


fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    
    let config = Config::parse(&args);

    match config {
        Ok(cfg)=>{
            adows::run(cfg);
        },
        Err(e)=>{
            println!("{}",e.message);
        }
    };

 
}

// for tests
// fn main(){
//     use std::path::{Path,PathBuf};

//     // let path = Path::new("").join("www").join("../style.css");
//     // let path = Path::new("./style.css");
//     // let path = path.strip_prefix("./").unwrap_or(path);
//     // dbg!(&path);

// }
