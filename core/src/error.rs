use std::io;

pub trait ToIoError {
    fn to_io(self) -> io::Error;
}

impl ToIoError for mongodb::error::Error {
    fn to_io(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }
}
