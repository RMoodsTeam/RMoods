use crate::report::report_error::ReportError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Database error: {0}")]
pub enum DbError {
    SqlxError(#[from] sqlx::Error),
    InvalidStateError(#[from] ReportError),
}
