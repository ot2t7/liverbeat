mod net;
use net::Server;

fn main() {
    let server = Server::new();
    println!("{} test", server.get_hello());
}
