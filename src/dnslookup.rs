use std::env;
use std::net::{ToSocketAddrs, SocketAddr};

pub fn dnslookup() -> Vec<SocketAddr>{
    let mut ip_addrs = vec![];
    for argument in env::args().skip(1) {
        let arg = String::from(&argument);
        let mut with_port = String::from(&arg);
        with_port.push_str(":80");
        let addrs = with_port.to_socket_addrs().unwrap();
      
        for addr in addrs { 
           ip_addrs.push(addr);
        }
    }
    return ip_addrs;
}