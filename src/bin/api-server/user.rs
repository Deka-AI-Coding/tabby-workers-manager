use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use actix_web::{
    dev::Payload, http::header::ToStrError, web, FromRequest, HttpRequest, ResponseError,
};
use thiserror::Error;

use crate::config::Config;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Auth header is missing")]
    NoHeader,
    #[error("No user found for this key")]
    NoUser,

    #[error("Cannot get ket from the server")]
    FailedToGetAuthKey,

    #[error("Cannot convert to string: {0}")]
    ToSTringError(#[from] ToStrError),
}

impl ResponseError for UserError {}

pub struct AuthenticatedUser {}

impl FromRequest for AuthenticatedUser {
    type Error = UserError;
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedUser, UserError>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header = req
            .headers()
            .get("Authorization")
            .ok_or(UserError::NoHeader)
            .map(|h| h.clone());

        let allowed_key = req
            .app_data::<web::Data<Config>>()
            .ok_or(UserError::FailedToGetAuthKey)
            .map(|k| k.key.clone());

        Box::pin(async move {
            let key = header?;
            let key_str = key.to_str()?;

            if key_str != allowed_key? {
                return Err(UserError::NoUser);
            }

            Ok(AuthenticatedUser {})
        })
    }
}
