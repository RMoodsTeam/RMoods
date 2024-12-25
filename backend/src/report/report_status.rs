use crate::validation::validation_error::ValidationError;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ReportStatus {
    Success,
    InProgress,
    Error(String),
}

impl ReportStatus {
    pub fn from_booleans(
        success: bool,
        in_progress: bool,
        error: bool,
        error_msg: Option<String>,
    ) -> Result<Self, ValidationError> {
        match (success, in_progress, error, error_msg) {
            (true, _, _, _) => Ok(Self::Success),
            (_, true, _, _) => Ok(Self::InProgress),
            (_, _, true, Some(msg)) => Ok(Self::Error(msg)),
            (_, _, _, _) => Err(ValidationError::Invalid(
                "Invalid report status".to_string(),
            )),
        }
    }

    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error(msg) => Some(msg),
            _ => None,
        }
    }
}
