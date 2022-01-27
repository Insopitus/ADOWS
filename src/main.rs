use std::env;

fn main() {
    let path = env::args().skip(1).next().unwrap_or_else(|| {
        // set current directory as target if param is not provided
        env::current_dir()
            .expect("Failed to get current directory.")
            .to_string_lossy()
            .to_string()
    });
    let port = 8080;
    adows::run(port, path);
}
