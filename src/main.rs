mod net;
use net::Server;

fn main() {
    let server = Server::new();
    println!("{} ", server.get_hello());
}