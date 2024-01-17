use chrono::NaiveDate;

pub fn assert_argument_not_empty(value: String, message: String) {
    if value == "" {
        panic!("{message}");
    }
}

pub fn assert_argument_date_format(value: String, message: String) {
    if NaiveDate::parse_from_str(&value, "%Y-%m-%d").is_err() {
        panic!("{message}");
    }
}

pub fn assert_argument_not_negative(value: f32, message: String) {
    if value < 0.0 {
        panic!("{message}");
    }
}