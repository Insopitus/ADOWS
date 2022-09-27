use std::{env, path::PathBuf};

const HELP_MESSAGE: &str = "
    ADOWS start a local server to serve your static files.
    Usage:
        adows [OPTIONS] [DIRECTORY] [PORT]

    Options:
        -v, --version       print the current version of adows and exit.
        -h, --help          print this message and exit.
        -c, --cross-origin  send cross-origin header field.
        -b, --open-browser  open the browser on server start.
";
// TODO extra options:
// -k, --keep-alive    allow keep-alive connections (not implemented yet).
// -g, --gzip          if request header accepts gz, adows will send [filename].[ext].gz instead
// --brotli            if request header accepts br, adows will send [filename].[ext].br instead 
// -s, --silent        will print no output to stdout

/// default port
const PORT: u16 = 8080;

/// default value for open_browser
const OPEN_BROWSER: bool = false;

/// default value for cross_origin/cors
const CROSS_ORIGIN: bool = false;

#[derive(PartialEq,Eq, Debug)]
pub struct Config {
    pub port: u16,
    pub dir: PathBuf,
    pub open_browser: bool,
    pub cross_origin: bool,
}

impl Config {
    /// parse the args (collected as array slice of strings) to create a config
    pub fn parse(args: &[String]) -> Result<Self, Error> {
        let mut port = PORT;
        let dir: PathBuf;
        let mut open_browser = OPEN_BROWSER;
        let mut cross_origin = CROSS_ORIGIN;

        let mut options = Vec::new();
        let mut commands = Vec::new();

        for s in args {
            if let Some(stripped) = s.strip_prefix('-'){
                options.push(stripped);
            }else{
                commands.push(s);
            }
        }
        // turn all long options to their short form
        let mut options_shorten = Vec::with_capacity(options.len());
        for s in options {
            if s.starts_with('-') {
                options_shorten.push(match s {
                    "-version" => "v",
                    "-open-browser" => "b",
                    "-cross-origin" => "c",
                    "-help" => "h",
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidOption(s.to_string()),
                            format!("Invalid option: -{}", s),
                        ))
                    }
                })
            } else {
                options_shorten.push(s);
            }
        }
        // handle options like "-bc"
        let mut options_flatten = Vec::with_capacity(options_shorten.len());
        for s in options_shorten {
            options_flatten.extend(s.chars());
        }
        for c in options_flatten {
            match c {
                'v' => Config::print_version(),
                'h' => Config::print_help(),
                'c' => {
                    cross_origin = true;
                }
                'b' => {
                    open_browser = true;
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidOption(c.to_string()),
                        format!("Invalid option: -{}", c),
                    ));
                }
            }
        }
        match commands.len() {
            0 => {
                dir = env::current_dir().map_err(|_| {
                    Error::new(
                        ErrorKind::CannotGetCurrentDir,
                        "Could not get current directory".to_string(),
                    )
                })?;
            }
            1 => {
                dir = PathBuf::from(commands[0]);
            }
            2 => {
                dir = PathBuf::from(commands[0]);
                port = commands[1].parse::<u16>().map_err(|_| {
                    Error::new(
                        ErrorKind::InvalidPort(commands[1].to_string()),
                        format!("Invalid port: {}", commands[1]),
                    )
                })?;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidCommand,
                    "Too many arguments".to_string(),
                ));
            }
        }

        Ok(Self {
            port,
            dir,
            open_browser,
            cross_origin,
        })
    }
    fn print_version() {
        println!("adows {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    fn print_help() {
        println!("{}", HELP_MESSAGE);
        std::process::exit(0);
    }
}

/// CLI parsing errors
#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}
impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { message, kind }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    CannotGetCurrentDir,
    InvalidPort(String),
    InvalidOption(String),
    InvalidCommand,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let options = Config::parse(&["./".to_string(), "8080".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: PORT,
                dir: PathBuf::from("./"),
                open_browser: OPEN_BROWSER,
                cross_origin: CROSS_ORIGIN,
            }
        );
    }
    #[test]
    fn basic_2() {
        let options = Config::parse(&["/dev".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: PORT,
                dir: PathBuf::from("/dev"),
                open_browser: OPEN_BROWSER,
                cross_origin: CROSS_ORIGIN
            }
        );
    }
    #[test]
    fn options() {
        let options = Config::parse(&["-c".to_string(), "--open-browser".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: PORT,
                dir: env::current_dir().unwrap(),
                open_browser: true,
                cross_origin: true
            }
        );
    }
    #[test]
    fn options_2() {
        let options = Config::parse(&["-cb".to_string()]).unwrap();
        assert_eq!(
            options,
            Config {
                port: PORT,
                dir: env::current_dir().unwrap(),
                open_browser: true,
                cross_origin: true
            }
        );
    }
}
