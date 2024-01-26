use chrono::DateTime;

use crate::core::domain::base::domain_error::DomainError;

pub fn assert_argument_not_empty(value: String) -> Result<(), DomainError> {
    if value.is_empty() {
        Err(DomainError::Empty)
    } else {
        Ok(())
    }
}

pub fn assert_argument_timestamp_format(value: String) -> Result<(), DomainError> {
    let format = "%Y-%m-%d %H:%M:%S%.6f";
    match DateTime::parse_from_str(&value, format) {
        Ok(_) => Ok(()),
        Err(_) => Err(DomainError::Invalid(value)),
    }
}

pub fn assert_argument_not_negative(value: f32) -> Result<(), DomainError> {
    if value < 0.0 {
        Err(DomainError::NonPositive)
    } else {
        Ok(())
    }
}
