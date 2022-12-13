use std::{net::TcpStream, io::{Read, BufReader, BufWriter}};

pub struct Connection {
    pub stream : TcpStream,
}

impl Connection {
    pub fn new(stream : TcpStream) -> Connection {
        Connection { stream: (stream) }
    }
}
