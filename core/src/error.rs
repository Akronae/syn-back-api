use std::io;

pub type SafeError = Box<dyn std::error::Error + Send + Sync>;

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

pub trait MapToIoError<T> {
    fn map_err_to_io(self) -> Result<T, io::Error>;
}

impl<T> MapToIoError<T> for Result<T, SafeError> {
    fn map_err_to_io(self) -> Result<T, io::Error> {
        self.map_err(|e| e.to_io())
    }
}
