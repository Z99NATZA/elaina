#![allow(unreachable_patterns)]

use std::net::TcpListener;
use std::io::Read;

use crate::app::AppResult;
use crate::http::{Method, Request, Response, StatusCode};

#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    
    pub fn run(&self) -> AppResult<()> {
        println!("\n----------------------------------------");
        println!("App running on: {}", self.addr);
        println!("----------------------------------------\n");

        let listener = TcpListener::bind(&self.addr)?;

        for tcp_stream in listener.incoming() {
            let mut stream = tcp_stream?;
            let mut buf: [u8; 1024] = [0; 1024];

            stream.read(&mut buf)?;
            
            let req = Request::try_from(&buf[..])?;
            println!("{req:#?}");
            
            let res = match req.method() {
                Method::GET => match req.path().as_ref() {
                    "/" => {
                        Response::new(StatusCode::Ok, Some("Home".to_string()))
                    }
                    "/hello" => {
                        Response::new(StatusCode::Ok, Some("Hello".to_string()))
                    }
                    _ => {
                        Response::new(StatusCode::NotFound, None)
                    }
                }
                Method::POST => {
                    println!("POST request received");
                    Response::new(StatusCode::Ok, None)
                }
                Method::PUT => {
                    println!("PUT request received");
                    Response::new(StatusCode::Ok, None)
                }
                Method::DELETE => {
                    println!("DELETE request received");
                    Response::new(StatusCode::Ok, None)
                }
                _ => {
                    Response::new(StatusCode::NotFound, None)
                }
            };
            
            res.send(&mut stream)?;
        }

        Ok(())
    }
}
