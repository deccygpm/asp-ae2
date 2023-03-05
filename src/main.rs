mod dnslookup;


use std::io::{Write, Read};
use std::net::{TcpStream, IpAddr};
use std::thread;
use std::sync::mpsc::channel;

fn main(){
    let dns_addresses = dnslookup::dnslookup();

    let (connection_sender, connection_receiver) = channel::<TcpStream>();

    let mut connection_attempts_threads = vec![];

    for _address in &dns_addresses {
        let (sender, receiver) = channel();
        let con_sender = connection_sender.clone();
        connection_attempts_threads.push((sender, thread::spawn(move|| {
            let message: IpAddr = receiver.recv().unwrap(); 
            let mut addr = message.to_string();
            addr.push_str(":80");
            if let Ok(stream) = TcpStream::connect(addr) {
                con_sender.send(stream).unwrap();
            } 
        })));
    }

    let connected_client_thread = thread::spawn(move|| {
        let mut stream = connection_receiver.recv().unwrap();
        println!("{:?}", stream.peer_addr().unwrap());
        let mut request = String::new();
        request.push_str(format!("GET / HTTP/1.1\r\n").as_str());
        request.push_str(format!("Host: {:?}\r\n", stream.peer_addr().unwrap()).as_str());
        request.push_str("Connection: close");
        request.push_str("\r\n");
        request.push_str("\r\n");
        let request_bytes = request.as_bytes();
        stream.write_all(request_bytes).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("{}", response);  
    });

    for i in 0..dns_addresses.len() {
        connection_attempts_threads[i].0.send(dns_addresses[i].ip()).unwrap();
    }

    connected_client_thread.join().unwrap();
    for t in connection_attempts_threads{
        t.1.join().unwrap();
    }
}
 
  

  

