#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::Error::Custom(
            format!("{}",format_args!($($arg)*)));
    }};
}
