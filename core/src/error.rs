use std::io;

pub trait ToIoError {
    fn to_io(self) -> io::Error;
}

impl ToIoError for mongodb::error::Error {
    fn to_io(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }
}

impl ToIoError for Box<dyn std::error::Error + Send + Sync> {
    fn to_io(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}
