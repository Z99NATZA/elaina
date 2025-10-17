#[derive(Debug)]
pub enum AppError {
    InvalidRequest,
    InvalidPotocal,
    InvalidMethod,
    IoError(String),
    Utf8Error(String),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value.to_string())
    }
}