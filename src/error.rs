use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("worker not found: {0}")]
    WorkerNotFound(String),

    #[error("docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
}

impl ResponseError for Error {}

pub type Result<T> = core::result::Result<T, Error>;
