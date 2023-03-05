use std::env;
use std::net::ToSocketAddrs;

fn main() {

    for argument in env::args().skip(1) {
        let arg = String::from(&argument);
        let mut with_port = String::from(&arg);
        with_port.push_str(":80");
        let addrs = with_port.to_socket_addrs().unwrap();
        for addr in addrs {
            let ip;
            if addr.is_ipv4() {
                ip = "IPv4";
            } else {
                ip = "IPv6";
            }
            println!("{} {} {:?}", argument, ip, addr.ip());
        }
        
    }
}