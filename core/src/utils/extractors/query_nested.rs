use std::{fmt, ops};

use std::future::{ready, Ready};

use actix_web::{dev::Payload, FromRequest, HttpRequest};

use serde::de::DeserializeOwned;
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueryNested<T>(pub T);

impl<T> ops::Deref for QueryNested<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for QueryNested<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for QueryNested<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: DeserializeOwned> FromRequest for QueryNested<T> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            serde_qs::from_str::<T>(req.query_string())
                .map(|val| QueryNested(val))
                .map_err(|e| {
                    debug!(
                        "Failed during Query extractor deserialization. \
         Request path: {:?}",
                        req.path()
                    );

                    actix_web::error::ErrorBadRequest(e)
                }),
        )
    }
}
