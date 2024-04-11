pub use std::fmt::Debug;
pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String
}

pub fn create_date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            return Err(FormError::new(
                "first_name".to_string(),
                self.first_name.to_string(),
                "No user name".to_string()))
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "At least 8 characters".to_string()))
        }
        if self.password.
            chars().
            filter(|x| x.is_ascii() && !x.is_alphanumeric()).
            collect::<Vec<char>>().
            len() == 0 ||
            self.password.
            chars().
            filter(|x| x.is_ascii_alphabetic()).
            collect::<Vec<char>>().
            len() == 0||
            self.password.
            chars().
            filter(|x| x.is_numeric()).
            collect::<Vec<char>>().
            len() == 0 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "Combination of different ASCII character types \
                     (numbers, letters and none alphanumeric characters)".to_string()))
        }

        return Ok(vec!["Valid first name", "Valid password"])
    }
}
