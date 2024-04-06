use server::Server;
use http::request;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
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
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: String,
            // super는 Parent Module를 참조할 수 있음
            method: Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE
        }        
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/