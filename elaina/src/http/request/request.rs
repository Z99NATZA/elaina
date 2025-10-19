use crate::app::{AppError, AppResult};
use crate::http::request::query::Query;
use crate::http::Method;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    query: Option<Query>,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = AppError;
    
    fn try_from(value: &[u8]) -> AppResult<Self> {
        let full_req = str::from_utf8(&value)?;

        let mut req = full_req.split_whitespace();

        let req_method = req.next().ok_or(AppError::InvalidRequest)?;
        let req_url = req.next().ok_or(AppError::InvalidRequest)?;
        let req_protocal = req.next().ok_or(AppError::InvalidRequest)?;

        if req_protocal != "HTTP/1.1" {
            return Err(AppError::InvalidPotocal);
        }

        let method: Method = req_method.parse()?;

        let mut path = req_url;

        let query = match req_url.find("?") {
            Some(value) => {
                let q = &req_url[value+1..];
                path = &req_url[..value];
                Some(Query::from(q))
            }
            None => None
        };

        Ok(Self {
            method,
            path: path.to_string(),
            query,
        })
    }
}