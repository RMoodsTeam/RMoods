use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Invalid report state: {0}")]
    InvalidState(String),
}
