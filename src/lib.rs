#![allow(dead_code)]

#[macro_use]
mod macros;

pub mod day1;
mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
            }
        }
    }

    impl std::error::Error for Error {}
}
