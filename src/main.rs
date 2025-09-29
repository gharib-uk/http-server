use std::net::{TcpListener, TcpStream};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};
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
                            //Request::try_from(&buffer as &[u8]).unwrap(); // Tell the compiler treat &[u8;1024] as &[u8]
                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {}
                                Err(e) => {println!("Failed to parse request: {}", e);}
                            }
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

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Request, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
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


enum ParseError {
    InvalidMethod,
    InvalidQuery,
    InvalidHeader,
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidMethod => "Invalid method",
            ParseError::InvalidQuery => "Invalid query",
            ParseError::InvalidHeader => "Invalid header",
            ParseError::InvalidRequest => "Invalid request",
            ParseError::InvalidEncoding => "Invalid encoding",
            ParseError::InvalidProtocol => "Invalid protocol",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}


fn main() {
    //let addr = String::from("127.0.0.1:8080");
    let srv = Server::new("127.0.0.1:9980".to_string());
    srv.run();
}
