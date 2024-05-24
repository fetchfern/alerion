use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("docker error: {0}")]
    Docker(#[from] docker::DockerError),
    #[error("malformed response from the Docker API")]
    MalformedResponse,
    #[error("panel remote API error: {0}")]
    RemoteApi(#[from] remote::ResponseError),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("action cannot be performed at this moment")]
    Conflict,
}

pub mod docker;
pub mod pool;
pub mod remote;
pub mod server;
