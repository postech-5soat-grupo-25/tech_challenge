use chrono::NaiveDate;

use crate::core::domain::base::domain_error::DomainError;

pub fn assert_argument_not_empty(value: String) -> Result<(), DomainError> {
    if value.is_empty() {
        Err(DomainError::Empty)
    } else {
        Ok(())
    }
}

pub fn assert_argument_date_format(value: String) -> Result<(), DomainError> {
    match NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
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
