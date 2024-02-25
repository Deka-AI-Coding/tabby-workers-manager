use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("worker not found: {0}")]
    WorkerNotFound(String),

    #[error("docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::WorkerNotFound(_) => StatusCode::NOT_FOUND,
            Error::DockerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
