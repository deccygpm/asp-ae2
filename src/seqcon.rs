use std::net::{TcpStream, SocketAddr};
use std::io::{Write, Read};

pub fn seqcon(addrs: Vec<SocketAddr>) -> std::io::Result<()> {
    
    for addr in addrs{
    let mut stream = TcpStream::connect(addr)?; 
       
    println!("{}", addr.ip());
    let mut request = String::new();
    request.push_str(format!("GET / HTTP/1.1\r\n").as_str());
    request.push_str(format!("Host: {}\r\n", addr).as_str());
    request.push_str("Connection: close");
    request.push_str("\r\n");
    request.push_str("\r\n");
    let request_bytes = request.as_bytes();
    stream.write_all(request_bytes).expect("handle this later");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("idk");
    println!("{}", response);   
    break; 
     
    }
    return Ok(());
}
   
