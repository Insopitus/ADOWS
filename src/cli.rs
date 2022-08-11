use std::{
    env,
    path::PathBuf,
};

const HELP_MESSAGE: &'static str = "
    ADOWS start a local server to serve your static files.
    Options:

    -p, --port=PORT     set the port used. Default one is 8080;
    -d, --dir=DIRECTORY set the directory to serve. Default one is the current directory;
    -c, --cross-origin  allow cross-origin requests;
    -b, --browser       open the browser on server start.
    -h, --help          print help message and exit.
";

const DEFAULT_PORT:u16 = 8080;

const DEFAULT_OPEN_BROWSER:bool = false;

const DEFAULT_CROSS_ORIGIN:bool = false;

// TODO revamp the cli settings should be allowed to use like `rm -rf some_dir` => `adows -bc some_dir 8088`

/// cli options struct
///
/// `--port`, `-p` => `u32` : the port adows uses
///
/// `--dir`, `-d` => `String` : the dir of the files that you want adows
/// to host (use current directory by default)
///
/// `--help`,`-h` prints out the available args
///
#[derive(PartialEq, Debug)]
pub struct Config {
    pub port: u16,
    pub dir: PathBuf,
    pub browser: bool,
    pub cross_origin: bool,
}

enum Pairing {
    Port,
    Dir,
    None,
}

impl Config {
    /// parse the args (collected as array slice of strings) to create a config
    pub fn parse(args: &[String]) -> Self {
        let mut port = DEFAULT_PORT;
        let mut dir = env::current_dir()
            .expect("Failed to get current directory.");
        let mut browser = DEFAULT_OPEN_BROWSER;
        let mut cross_origin = DEFAULT_CROSS_ORIGIN;

        let mut paring = Pairing::None;

        for s in args {
            match s.as_str() {
                "--help" | "-h" => {
                    println!("{}", HELP_MESSAGE);
                    std::process::exit(0);
                }
                "--port" | "-p" => {
                    paring = Pairing::Port;
                }
                "--dir" | "-d" => {
                    paring = Pairing::Dir;
                }
                "--browser" | "-b" => {
                    browser = true;
                }
                "--cross-origin" | "-c" => {
                    cross_origin = true;
                }
                st => match paring {
                    Pairing::Dir => {
                        dir = PathBuf::from(st);
                    }
                    Pairing::Port => {
                        port = st.parse().expect("Unparsable port");
                    }
                    Pairing::None => {}
                },
            }
        }
        Self {
            port,
            dir,
            browser,
            cross_origin,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let options = Config::parse(&["--port".to_string(), "8080".to_string()]);
        assert_eq!(
            options,
            Config {
                port: DEFAULT_PORT,
                dir: env::current_dir().unwrap(),
                browser: DEFAULT_OPEN_BROWSER,
                cross_origin:DEFAULT_CROSS_ORIGIN,
            }
        );
    }
    #[test]
    fn basic_2() {
        let options = Config::parse(&["--dir".to_string(), "/dev".to_string()]);
        assert_eq!(
            options,
            Config {
                port: DEFAULT_PORT,
                dir: PathBuf::from("/dev"),
                browser: DEFAULT_OPEN_BROWSER,
                cross_origin:DEFAULT_CROSS_ORIGIN
            }
        );
    }
    #[test]
    fn basic_3() {
        let options = Config::parse(&[
            "--dir".to_string(),
            "/dev".to_string(),
            "--port".to_string(),
            "3000".to_string(),
        ]);
        assert_eq!(
            options,
            Config {
                port: 3000,
                dir: PathBuf::from("/dev"),
                browser: DEFAULT_OPEN_BROWSER,
                cross_origin: DEFAULT_CROSS_ORIGIN
            }
        );
    }
}
