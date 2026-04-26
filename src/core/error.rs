use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    Runtime,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Runtime => write!(f, "Runtime Error"),
        }
    }
}
