use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use webser::Connection;

const CRLF : &str = "\r\n";
const START : &str=  ".";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for client in listener.incoming() {
        let client = client.unwrap();
        std::thread::spawn(||handle_client(client));
    }
}

fn handle_client(client : TcpStream) {
    let mut c : Connection = webser::Connection::new(client);
    loop {
        let r =  handle_request(& mut c);    
        if (!r) {
            break;
        }
    }
}

fn handle_request(c : & mut Connection) -> bool {
    let mut bytes : [u8 ; 4096]= [0; 4096]; 
    
    let size = c.stream.read(& mut bytes).unwrap();
    let as_str = std::str::from_utf8(bytes.as_slice()).unwrap();
    println!("Packet :\n {} \n", as_str);    

    let mut url = String::new();
    let mut url_flag = false;
    for i in 0..size {
        if (as_str.chars().nth(i).unwrap() == ' ' && url_flag == false) {
            url_flag = true;
        } else if (as_str.chars().nth(i).unwrap() == ' ' && url_flag == true) {
            break;
        } else if (url_flag == true){
            url.push(as_str.chars().nth(i).unwrap());
        }
    }

    handle_url(&mut c.stream, url.as_str())
}

fn matched_url(bytes : & [u8] , url : & str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", url, CRLF);
    bytes.starts_with(s.as_bytes())
}

fn handle_url(b : & mut TcpStream, url: & str) -> bool {
    let mut s = String::new();
    if (url.eq("/") == true) {
        s.push_str("./index/index.html");
    } else {
        s.push_str(START);
        s.push_str(url);
        println!("Url : {}", s);
    }

    let open_res = File::open(s);
    match open_res {
       Err(e) => {
        handle_404(b);
        return false;
       },
       Ok(_) => {} 
    } 

    let mut f = open_res.unwrap();

    let mut content_vec = Vec::<u8>::new();

    match f.read_to_end(&mut content_vec) {
        Err(e) => {
            println!("Read File Error {}", e);
        },
        Ok(read_size) => {}
    }


    println!("Content len {}", content_vec.len());
    let http_packet = format!("HTTP/1.1 200 Ok\r\nContent-Length : {}\r\n\r\n", content_vec.len()); 
    b.write(http_packet.as_bytes());
    // send once
    b.write(content_vec.as_slice());
    
    b.flush();
    true
}

fn handle_404(b : & mut TcpStream) {
    let res_str = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
    b.write(res_str.as_bytes());
    b.flush();
    b.shutdown(std::net::Shutdown::Both);
}