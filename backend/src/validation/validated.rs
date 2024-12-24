use crate::validation::validation_error::ValidationError;

pub trait Validated {
    fn validate(&self) -> Result<(), ValidationError>;
}
