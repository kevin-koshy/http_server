use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(self) {
        println!("Server started on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer= [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer)[..]{
                                Ok(request) => {},
                                Err(e) => println!{"Failed with an error {}", e}
                            }

                        },
                        Err(e)=> println!("Failed to read from Connection {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

        }

    }
}
