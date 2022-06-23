use std::env;


fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        // set current directory as target if param is not provided
        // "www".to_string()
        env::current_dir()
            .expect("Failed to get current directory.")
            .to_string_lossy()
            .to_string()
    });
    let port = 8080;
    adows::run(port, path);
}

// for tests
// fn main(){
//     use std::path::{Path,PathBuf};

//     // let path = Path::new("").join("www").join("../style.css");
//     // let path = Path::new("./style.css");
//     // let path = path.strip_prefix("./").unwrap_or(path);
//     // dbg!(&path);

// }
