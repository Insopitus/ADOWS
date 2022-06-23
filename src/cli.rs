use std::{
    env::Args,
    io,
    path::{Path, PathBuf},
};

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
}

enum Paring {
    Port,
    Dir,
    None,
}

impl Config {
    /// parse the args (collected as array slice of strings) to create a config
    pub fn parse(args: &[String]) -> io::Result<Self> {
        let mut args = args.iter();
        let mut port = 8080;
        let mut dir = std::env::current_dir()?;
        let mut paring = Paring::None;
        loop {
            match args.next() {
                Some(s) => match s.as_str() {
                    "--help" | "-h" => {
                        println!("insert help text here");
                        std::process::exit(0);
                    }
                    "--port" | "-p" => {
                        paring = Paring::Port;
                        continue;
                    }
                    "--dir" | "-d" => {
                        paring = Paring::Dir;
                        continue;
                    }
                    st => {
                        match paring {
                            Paring::Dir => {
                                dir = PathBuf::from(st);
                            }
                            Paring::Port => {
                                port = st.parse().expect("Unparsable port");
                            }
                            Paring::None => {
                                continue;
                            }
                        }
                        continue;
                    }
                },
                None => {
                    break;
                }
            };
        }
        Ok(Self { port, dir })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let options = Config::parse(&["--port".to_string(), "8080".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: 8080,
                dir: std::env::current_dir().unwrap(),
            }
        );
    }
    #[test]
    fn basic_2() {
        let options = Config::parse(&["--dir".to_string(), "/dev".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: 8080,
                dir: PathBuf::from("/dev"),
            }
        );
    }
    #[test]
    fn basic_3() {
        let options = Config::parse(&[
            "--dir".to_string(), 
            "/dev".to_string(),
            "--port".to_string(),
            "3000".to_string()
            ]).unwrap();
        assert_eq!(
            options,
            Config {
                port: 3000,
                dir: PathBuf::from("/dev"),
            }
        );
    }
}
