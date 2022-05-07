use std::path::{PathBuf, Path};

/// cli options parser
///
/// `--port` `-p` => `u32` : the port adows uses
///
/// `--dir` `-d` => `String` : the dir of the files that you want adows
/// to host (use current directory by default)
///
pub fn parse_cli(line: &str) -> Result<CliOptions,&str> {
   
    Ok(CliOptions {
        port: 0,
        dir: PathBuf::new(),
        threads: 4,
    })
}

#[derive(PartialEq, Debug)]
pub struct CliOptions {
    pub port: u64,
    pub dir: PathBuf,
    pub threads: usize,
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
