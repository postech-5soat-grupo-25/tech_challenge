pub fn assert_argument_not_empty(value: String, message: String) {
    if value == "" {
        panic!("{message}");
    }
}