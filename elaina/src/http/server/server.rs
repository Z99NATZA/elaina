use std::net::TcpListener;
use std::io::Read;

use crate::app::AppResult;

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

            let request = str::from_utf8(&buf)?;
            println!("{}", request);
        }

        Ok(())
    }
}
