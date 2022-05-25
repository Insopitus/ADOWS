use std::{
    env::Args,
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

impl Config {
    /// parse the args (collected as array slice of strings) to create a config
    fn parse(args: &[String]) {}
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn basic() {
//         let options = parse_cli("--port 8080");
//         assert_eq!(
//             options,
//             Ok(CliOptions {
//                 port: 8080,
//                 dir: PathBuf::new(),
//                 threads: 4
//             })
//         );
//     }

//     fn space_in_path() {
//         let options = parse_cli(r#"-d "a folder/index.html""#);
//     }
// }
