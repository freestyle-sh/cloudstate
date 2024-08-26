#[macro_export]
macro_rules! v8_string_key {
    ($scope:expr, $key:expr) => {
        v8::String::new($scope, $key).unwrap().into()
    };
}
