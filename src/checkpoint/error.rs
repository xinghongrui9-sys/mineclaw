use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckpointError {
    #[error("Checkpoint not found: {0}")]
    NotFound(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Checkpoint limit reached for session: {0}")]
    LimitReached(String),

    #[error("AgentFS error: {0}")]
    AgentFS(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid checkpoint data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, CheckpointError>;
