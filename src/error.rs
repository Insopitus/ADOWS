use std::fmt;
use std::io;
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::BadRequest => {
                write!(f, "Bad Request")
            }
            ErrorKind::InternalServerError => {
                write!(f, "Internal Server Error")
            }
            _ => {
                write!(f, "Internal Server Error")
            }
        }
    }
}
impl From<io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let new_kind = match e.kind() {
            io::ErrorKind::NotFound => ErrorKind::FileNotFound,
            io::ErrorKind::OutOfMemory => ErrorKind::OutOfMemory,
            io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            _ => {
                dbg!(&e);

                ErrorKind::InternalServerError
            }
        };
        Error { kind: new_kind } // TODO placeholder
    }
}
// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f,"Error happened")

//     }
// }

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq,Eq)]
pub enum ErrorKind {
    BadRequest,
    InternalServerError,
    FileNotFound,
    OutOfMemory,
    AddrInUse,
}
