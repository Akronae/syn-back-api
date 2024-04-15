use std::io::{self, Stderr};

use anyhow::anyhow;
use scraper::error::SelectorErrorKind;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

pub type SafeError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait IntoErr<T> {
    fn into_err(self) -> T;
}

impl IntoErr<io::Error> for mongodb::error::Error {
    fn into_err(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }
}

impl IntoErr<io::Error> for SafeError {
    fn into_err(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}

impl IntoErr<actix_web::Error> for SafeError {
    fn into_err(self) -> actix_web::Error {
        actix_web::error::ErrorInternalServerError(self.to_string())
    }
}

impl IntoErr<actix_web::Error> for anyhow::Error {
    fn into_err(self) -> actix_web::Error {
        actix_web::error::ErrorInternalServerError(self.to_string())
    }
}

impl IntoErr<SafeError> for mongodb::error::Error {
    fn into_err(self) -> SafeError {
        Box::new(Error(anyhow!(self)))
    }
}

impl IntoErr<SafeError> for SelectorErrorKind<'_> {
    fn into_err(self) -> SafeError {
        Box::new(Error(anyhow!(self.to_string())))
    }
}

impl IntoErr<SafeError> for io::Error {
    fn into_err(self) -> SafeError {
        Box::new(Error(anyhow!(self)))
    }
}

impl IntoErr<SafeError> for Stderr {
    fn into_err(self) -> SafeError {
        Box::new(Error(anyhow!(format!("{:?}", self))))
    }
}

impl IntoErr<SafeError> for anyhow::Error {
    fn into_err(self) -> SafeError {
        Box::new(Error(self))
    }
}

pub trait MapIntoErr<TRes, TErr> {
    fn map_into_err(self) -> Result<TRes, TErr>;
}

impl<TRes, TErrFrom, TErrInto> MapIntoErr<TRes, TErrInto> for Result<TRes, TErrFrom>
where
    TErrFrom: IntoErr<TErrInto>,
{
    fn map_into_err(self) -> Result<TRes, TErrInto> {
        self.map_err(|e| e.into_err())
    }
}

pub trait MapErrIo<TRes> {
    fn map_err_io(self) -> Result<TRes, io::Error>
    where
        Self: Sized;
}

impl<TRes> MapErrIo<TRes> for Result<TRes, SafeError> {
    fn map_err_io(self) -> Result<TRes, io::Error> {
        self.map_into_err()
    }
}

pub trait MapErrActix<TRes> {
    fn map_err_actix(self) -> Result<TRes, actix_web::Error>;
}

impl<TRes> MapErrActix<TRes> for Result<TRes, SafeError> {
    fn map_err_actix(self) -> Result<TRes, actix_web::Error> {
        self.map_into_err()
    }
}

impl<TRes> MapErrActix<TRes> for Result<TRes, anyhow::Error> {
    fn map_err_actix(self) -> Result<TRes, actix_web::Error> {
        self.map_into_err()
    }
}

pub trait MapErrSafe<TRes> {
    fn map_err_safe(self) -> Result<TRes, SafeError>;
}

impl<TRes> MapErrSafe<TRes> for Result<TRes, mongodb::error::Error> {
    fn map_err_safe(self) -> Result<TRes, SafeError> {
        self.map_into_err()
    }
}

impl<TRes> MapErrSafe<TRes> for Result<TRes, io::Error> {
    fn map_err_safe(self) -> Result<TRes, SafeError> {
        self.map_into_err()
    }
}

impl<TRes> MapErrSafe<TRes> for Result<TRes, anyhow::Error> {
    fn map_err_safe(self) -> Result<TRes, SafeError> {
        self.map_into_err()
    }
}
