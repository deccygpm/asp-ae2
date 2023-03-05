mod dnslookup;
mod seqcon;

fn main(){
    let dns_addresses = dnslookup::dnslookup();
    seqcon::seqcon(dns_addresses).expect("Connections failed");
}