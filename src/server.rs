use crate::http::request::Request;
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // = Request::try_from(&buffer[..]);
                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {

                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
                
                // default와 같은 역할, match에 enum과 같은 type이 할당되었을 경우 default 역할을함
                // _ => println!("Unkown"), 
            }
        }
    }
}