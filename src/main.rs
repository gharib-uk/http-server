use std::net::{TcpListener, TcpStream};
use std::io::{Read, };
struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Server {
        Server { address }
    }

    fn run(self) {
        println!("Server running on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addrSock)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request Msg Received: {}",
                                     String::from_utf8_lossy(&buffer[..]));
                        }

                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                        }
                    }
                }

                Err(e) => {
                    println!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}

fn main() {
    //let addr = String::from("127.0.0.1:8080");
    let srv = Server::new("127.0.0.1:9980".to_string());
    srv.run();
}
