use std::{
    future::{ready, Ready},
    str::FromStr,
};

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathObjectId {
    pub extracted: ObjectId,
}

impl FromRequest for PathObjectId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            req.match_info()
                .get("id")
                .and_then(|id| ObjectId::from_str(id).ok())
                .map(|extracted| PathObjectId { extracted })
                .ok_or_else(|| {
                    actix_web::error::ErrorBadRequest(format!(
                        "Invalid ObjectId: {}",
                        req.match_info().get("id").unwrap_or("")
                    ))
                }),
        )
    }
}
