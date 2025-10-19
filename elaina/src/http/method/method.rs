use std::str::FromStr;

use crate::app::{AppError, AppResult};

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl FromStr for Method {
    type Err = AppError;

    fn from_str(s: &str) -> AppResult<Self> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(AppError::InvalidMethod),
        }
    }
}